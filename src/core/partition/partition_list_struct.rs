// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;
use std::ops::Index;
use std::ops::IndexMut;

// From this library
use crate::core::errors::PartitionListError;
use crate::core::partition::Partition;
use crate::core::partition::PartitionIter;
use crate::core::partition::PartitionIterMut;
use crate::owning_mut_from_ptr;
use crate::owning_ref_from_ptr;

/// Collection of [`Partition`](crate::core::partition::Partition)s.
///
/// `PartitionList` is a collection of partitions. It is not connected to a partition table, as
/// such. Any change to a list does not affect a device's data, neither in memory nor on disk.
///
/// # Examples
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use rsfdisk::partition::Partition;
/// use rsfdisk::partition::PartitionList;
///
/// fn main() -> rsfdisk::Result<()> {
///     let partition1 = Partition::builder()
///          .number(1)
///          .starting_sector(64)
///          .build()?;
///     let partition2 = Partition::builder()
///         .number(2)
///         .starting_sector(4096)
///         .build()?;
///     let partition3 = Partition::builder()
///         .number(3)
///         .starting_sector(8192)
///         .build()?;
///
///     let mut list = PartitionList::new()?;
///     list.push(partition1)?;
///     list.push(partition2)?;
///     list.push(partition3)?;
///
///     assert_eq!(list.len(), 3);
///
///     let last = list.pop().unwrap();
///     assert_eq!(list.len(), 2);
///     assert_eq!(last.number(), Some(3));
///
///     for partition in list.iter() {
///         let partition_number = partition.number().unwrap();
///         assert!(partition_number > 0);
///     }
///
///     assert_eq!(list[0].size_in_sectors(), None);
///
///     for partition in list.iter_mut() {
///         partition.set_size_in_sectors(128)?;
///     }
///     assert_eq!(list[0].size_in_sectors(), Some(128));
///
///     assert_eq!(list[1].starting_sector(), Some(4096));
///
///     list[1].set_starting_sector(65536)?;
///     assert_eq!(list[1].starting_sector(), Some(65536));
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct PartitionList {
    pub(crate) inner: *mut libfdisk::fdisk_table,
    pub(crate) gc: Vec<*mut *mut libfdisk::fdisk_partition>,
}

impl PartitionList {
    #[doc(hidden)]
    /// Increments the list's reference counter.
    #[allow(dead_code)]
    pub(crate) fn incr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_ref_table(self.inner) }
    }

    #[doc(hidden)]
    /// Decrements the list's reference counter.
    #[allow(dead_code)]
    pub(crate) fn decr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_unref_table(self.inner) }
    }

    #[doc(hidden)]
    /// Borrows a `PartitionList` instance.
    pub(crate) fn borrow_ptr(ptr: *mut libfdisk::fdisk_table) -> PartitionList {
        let mut list = Self::from_ptr(ptr);

        // We are virtually ceding ownership of this instance which will be automatically
        // deallocated once it is out of scope, incrementing its reference counter protects it from
        // being freed prematurely.
        list.incr_ref_counter();

        list
    }

    #[doc(hidden)]
    /// Wraps a raw `libfdisk::fdisk_table` with a safe `PartitionList`.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut libfdisk::fdisk_table) -> PartitionList {
        Self {
            inner: ptr,
            gc: vec![],
        }
    }

    /// Creates a new `PartitionList`.
    pub fn new() -> Result<PartitionList, PartitionListError> {
        log::debug!("PartitionList::new creating a new `PartitionList` instance");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_table>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_new_table());
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `PartitionList` instance".to_owned();
                log::debug!(
                    "PartitionList::new {}. libfdisk::fdisk_new_table returned a NULL pointer",
                    err_msg
                );

                Err(PartitionListError::Creation(err_msg))
            }
            ptr => {
                log::debug!("PartitionList::new created a new `PartitionList` instance");
                let list = Self::from_ptr(ptr);

                Ok(list)
            }
        }
    }

    /// Appends a [`Partition`] to the back of a list.
    pub fn push(&mut self, mut partition: Partition) -> Result<(), PartitionListError> {
        log::debug!("PartitionList::push adding a partition to the list");

        // We are taking ownership of this partition which will be automatically deallocated once
        // this function ends, incrementing its reference counter prevents it from being
        // deallocated prematurely.
        partition.incr_ref_counter();

        let result = unsafe { libfdisk::fdisk_table_add_partition(self.inner, partition.inner) };

        match result {
            0 => {
                log::debug!("PartitionList::push added a partition to the list");

                Ok(())
            }
            code => {
                let err_msg = "failed to add partition to list".to_owned();
                log::debug!("PartitionList::push {}. libfdisk::fdisk_table_add_partition returned error code: {:?}", err_msg, code);

                Err(PartitionListError::Push(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Removes the partition at `index` from this `PartitionList`.
    fn remove_partition(&mut self, index: usize) -> Option<Partition> {
        self.get(index).and_then(|item| {

            // `libfdisk::fdisk_table_remove_partition` decrements the reference counter of a partition
            // after removing it from the table. To prevent the partition from being deallocated
            // prematurely, we increment its reference counter before calling
            // `fdisk_table_remove_partition`.
            let borrowed = Partition::borrow_ptr(item.inner);

            let result = unsafe { libfdisk::fdisk_table_remove_partition(self.inner, item.inner) };

            match result {
                0 => {
                    log::debug!("PartitionList::remove_partition removed partition");

                    Some(borrowed)
                }
                code => {
                    let err_msg = "failed to remove partition from `PartitionList`".to_owned();
                    log::debug!("PartitionList::remove_partition {}. libfdisk::fdisk_table_remove_partition returned error code: {:?}", err_msg, code);

                    // the item is not in the table, so we decrement its reference counter by
                    // dropping it to cancel out the incrementation performed by Partition::borrow_ptr
                    drop(borrowed);

                    None
                }
            }
        })
    }

    /// Removes the last element from a list and returns it, or w``None if it is empty.
    pub fn pop(&mut self) -> Option<Partition> {
        log::debug!("PartitionList::pop removing last item in `PartitionList`");

        if self.is_empty() {
            None
        } else {
            self.remove_partition(self.len() - 1)
        }
    }

    /// Removes the partition at `index` from this `PartitionList`.
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    pub fn remove(&mut self, index: usize) -> Partition {
        log::debug!(
            "PartitionList::remove removing partition at index: {:?}",
            index
        );

        let err_msg = format!("failed to find entry at index: {:?}", index);
        self.remove_partition(index)
            .ok_or(Err::<Partition, PartitionListError>(
                PartitionListError::IndexOutOfBounds(err_msg),
            ))
            .unwrap()
    }

    #[doc(hidden)]
    /// Release heap allocated Partition references.
    fn collect_garbage(&mut self) {
        while let Some(partition) = self.gc.pop() {
            let _ = unsafe { Box::from_raw(partition) };
        }
    }

    /// Clears the list, removing all partitions; partitions with a reference count at zero will
    /// be deallocated.
    ///
    /// **Note:** this function does not modify partition tables.
    pub fn clear(&mut self) -> Result<(), PartitionListError> {
        log::debug!("PartitionList::clear removing all elements in the list");

        let result = unsafe { libfdisk::fdisk_reset_table(self.inner) };

        match result {
            0 => {
                log::debug!("PartitionList::clear removed all elements in the list");
                self.collect_garbage();

                Ok(())
            }
            code => {
                let err_msg = "failed to remove all elements in the list".to_owned();
                log::debug!(
                    "PartitionList::clear {}. libfdisk::fdisk_reset_table returned error code: {:?}",
                    err_msg,
                    code
                );

                Err(PartitionListError::Clear(err_msg))
            }
        }
    }

    /// Returns the number of elements in the list, also referred to as its ‘length’.
    pub fn len(&self) -> usize {
        let length = unsafe { libfdisk::fdisk_table_get_nents(self.inner) };
        log::debug!("PartitionList::len number of elements: {:?}", length);

        length
    }

    #[doc(hidden)]
    fn get_partition(table: &Self, index: usize) -> Option<*mut libfdisk::fdisk_partition> {
        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_partition>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_table_get_partition(table.inner, index));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!("no partition a index: {:?}", index);
                log::debug!("PartitionList::get_partition {}. libfdisk::fdisk_table_get_partition returned a NULL pointer", err_msg);

                None
            }
            ptr => {
                log::debug!(
                    "PartitionList::get_partition got partition at index: {:?}",
                    index
                );

                Some(ptr)
            }
        }
    }

    /// Returns a iterator over the [`Partition`]s in the list.
    ///
    /// # Panics
    ///
    /// May panic if it fails to instantiate a new [`PartitionIter`].
    pub fn iter(&self) -> PartitionIter<'_> {
        PartitionIter::new(self).unwrap()
    }

    /// Returns a iterator over the [`Partition`]s in the list.
    ///
    /// # Panics
    ///
    /// May panic if it fails to instantiate a new [`PartitionIterMut`].
    pub fn iter_mut(&self) -> PartitionIterMut<'_> {
        PartitionIterMut::new(self).unwrap()
    }

    /// Returns a reference to the entry at `index` in the list, if `index` is not out of bounds.
    pub fn get(&self, index: usize) -> Option<&Partition> {
        log::debug!("PartitionList::get getting entry at index: {:?}", index);

        Self::get_partition(self, index).map(|ptr| owning_ref_from_ptr!(self, Partition, ptr))
    }

    /// Returns a mutable reference to the entry at `index` in the list, if `index` is not out of
    /// bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Partition> {
        log::debug!("PartitionList::get_mut getting entry at index: {:?}", index);

        Self::get_partition(self, index).map(|ptr| owning_mut_from_ptr!(self, Partition, ptr))
    }

    #[doc(hidden)]
    fn get_partition_by_partition_number(
        table: &Self,
        partition_number: usize,
    ) -> Option<*mut libfdisk::fdisk_partition> {
        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_partition>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_table_get_partition_by_partno(
                table.inner,
                partition_number,
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!(
                    "no partition matching identification number: {:?}",
                    partition_number
                );
                log::debug!("PartitionList::get_partition_by_partition_number {}. libfdisk::fdisk_table_get_partition_by_partno returned a NULL pointer", err_msg);

                None
            }
            ptr => {
                log::debug!("PartitionList::get_partition_by_partition_number got partition matching identification number: {:?}", partition_number);

                Some(ptr)
            }
        }
    }

    /// Returns a reference to the entry matching the given `partition_number`.
    pub fn get_by_partition_number(&self, partition_number: usize) -> Option<&Partition> {
        log::debug!("PartitionList::get_by_partition_number getting partition matching identification number: {:?}", partition_number);

        Self::get_partition_by_partition_number(self, partition_number)
            .map(|ptr| owning_ref_from_ptr!(self, Partition, ptr))
    }

    /// Returns a mutable reference to the entry matching the given `partition_number`.
    pub fn get_by_partition_number_mut(
        &mut self,
        partition_number: usize,
    ) -> Option<&mut Partition> {
        log::debug!("PartitionList::get_by_partition_number_mut getting partition matching identification number: {:?}", partition_number);

        Self::get_partition_by_partition_number(self, partition_number)
            .map(|ptr| owning_mut_from_ptr!(self, Partition, ptr))
    }

    // FIXME need upstream modification to accept a void* user_data parameter in the callback comparison function
    // see http://blog.sagetheprogrammer.com/neat-rust-tricks-passing-rust-closures-to-c
    //
    // /// Sorts the list with a comparator function.
    // ///
    // /// The comparator function must define a total ordering for the elements in the list. If the
    // /// ordering is not total, the order of the elements is unspecified.
    // ///
    // /// An order is a total order if it is (for all `a`, `b` and `c`):
    // /// - total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
    // /// - transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
    // pub fn sort_by<F>(&mut self, mut compare: F) -> Result<(), PartitionListError>
    // where
    //     F: FnMut(&Partition, &Partition) -> Ordering,
    // {
    //     log::debug!("PartitionList::sort_by sorting `PartitionList`");

    //     let cmp = |this_ptr: *mut libfdisk::fdisk_partition,
    //                other_ptr: *mut libfdisk::fdisk_partition|
    //      -> libc::c_int {
    //         let this = Partition::borrow_ptr(this_ptr);
    //         let other = Partition::borrow_ptr(other_ptr);

    //         match compare(&this, &other) {
    //             Ordering::Less => -1,
    //             Ordering::Equal => 0,
    //             Ordering::Greater => 1,
    //         }
    //     };

    //     let closure = Closure2::new(&cmp);
    //     //let function = closure.code_ptr();
    //     let function: unsafe extern "C" fn(
    //         *mut libfdisk::fdisk_partition,
    //         *mut libfdisk::fdisk_partition,
    //     ) -> libc::c_int = closure.code_ptr();

    //     // let result = unsafe {
    //     //     // FIXME explain
    //     //     // https://stackoverflow.com/a/70843628
    //     //     let fn_c_ptr: unsafe extern "C" fn(
    //     //         *mut libfdisk::fdisk_partition,
    //     //         *mut libfdisk::fdisk_partition,
    //     //     ) -> libc::c_int = std::mem::transmute(function);

    //     //     libfdisk::fdisk_table_sort_partitions(self.inner, Some(fn_c_ptr))
    //     // };

    //     let result = unsafe { libfdisk::fdisk_table_sort_partitions(self.inner, Some(function)) };

    //     match result {
    //         0 => {
    //             log::debug!("PartitionList::sort_by `PartitionList` sorted");

    //             Ok(())
    //         }
    //         code => {
    //             let err_msg = "failed to sort `PartitionList`".to_owned();
    //             log::debug!("PartitionList::sort_by {}. libfdisk::fdisk_table_sort_partitions returned error code: {:?}", err_msg, code);

    //             Err(PartitionListError::Sort(err_msg))
    //         }
    //     }
    // }

    /// Returns `true` when the list contains no elements.
    pub fn is_empty(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_table_is_empty(self.inner) == 1 };
        log::debug!("PartitionList::is_empty value: {:?}", state);

        state
    }

    /// Returns `true` if [`Partition`]s in the list are not kept in increasing order of their
    /// starting sectors.
    ///
    /// **Note:** this method skips [`Partition`]s that lack a starting sector value, or point to whole disks.
    pub fn is_not_in_increasing_order(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_table_wrong_order(self.inner) == 1 };
        log::debug!(
            "PartitionList::is_not_in_increasing_order value: {:?}",
            state,
        );

        state
    }
}

impl Index<usize> for PartitionList {
    type Output = Partition;

    fn index(&self, index: usize) -> &Self::Output {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn indexing_failed() -> ! {
            panic!("Index out of bounds");
        }

        let mut iter = PartitionIter::new(self).unwrap();
        match iter.nth(index) {
            Some(partition) => partition,
            None => indexing_failed(),
        }
    }
}

impl IndexMut<usize> for PartitionList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn indexing_failed() -> ! {
            panic!("Index out of bounds");
        }

        let mut iter = PartitionIterMut::new(self).unwrap();
        match iter.nth(index) {
            Some(partition) => partition,
            None => indexing_failed(),
        }
    }
}

impl AsRef<PartitionList> for PartitionList {
    #[inline]
    fn as_ref(&self) -> &PartitionList {
        self
    }
}

impl Drop for PartitionList {
    fn drop(&mut self) {
        log::debug!("PartitionList::drop deallocating `PartitionList` instance");

        unsafe { libfdisk::fdisk_unref_table(self.inner) }

        self.collect_garbage()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::core::partition::Guid;
    use crate::core::partition::Partition;
    use crate::core::partition::PartitionKind;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::cmp::Ordering;

    #[test]
    fn partition_list_a_new_list_is_empty() -> crate::Result<()> {
        let list = PartitionList::new()?;

        let actual = list.is_empty();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_add_an_item() -> crate::Result<()> {
        let mut list = PartitionList::new()?;

        let actual = list.is_empty();
        let expected = true;
        assert_eq!(actual, expected);

        let partition = Partition::builder().build()?;
        list.push(partition)?;

        let actual = list.is_empty();
        let expected = false;
        assert_eq!(actual, expected);

        let actual = list.len();
        let expected = 1;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_pop_the_last_item() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;

        let actual = list.len();
        let expected = 2;
        assert_eq!(actual, expected);

        let _ = list.pop();
        let actual = list.len();
        let expected = 1;
        assert_eq!(actual, expected);

        let actual = list.get(0).and_then(|p| p.number());
        let expected = Some(1);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_clear_a_list_of_all_its_items() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition2)?;
        list.push(partition1)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        list.clear()?;

        let actual = list.is_empty();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_remove_an_item() -> crate::Result<()> {
        let mut list = PartitionList::new()?;
        let partition = Partition::builder().build()?;
        list.push(partition)?;

        let actual = list.len();
        let expected = 1;
        assert_eq!(actual, expected);

        let _ = list.remove(0);
        let actual = list.is_empty();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_get_an_item_by_partition_number() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition2)?;
        list.push(partition1)?;
        list.push(partition3)?;

        let partition = list.get_by_partition_number(2).unwrap();

        let actual = partition.number();
        let expected = Some(2);
        assert_eq!(actual, expected);

        let actual = partition.starting_sector();
        let expected = Some(4096);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_test_items_have_starting_sectors_in_increasing_order() -> crate::Result<()>
    {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        //list.sort_by(|p1, p2| p1.compare_starting_sectors(p2))?;

        let actual = list.is_not_in_increasing_order();
        let expected = false;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_test_items_have_starting_sectors_not_in_increasing_order(
    ) -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition3)?;
        list.push(partition2)?;

        //list.sort_by(|p1, p2| p1.compare_starting_sectors(p2))?;

        let actual = list.is_not_in_increasing_order();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    #[ignore] // FIXME need upstream function modification
    fn partition_list_can_sort_by_starting_sector() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition3)?;
        list.push(partition2)?;
        list.push(partition1)?;

        //list.sort_by(|p1, p2| p1.compare_starting_sectors(p2))?;

        let actual = list.get(0).and_then(|p| p.starting_sector());
        let expected = Some(64);
        assert_eq!(actual, expected);

        let actual = list.get(1).and_then(|p| p.starting_sector());
        let expected = Some(4096);
        assert_eq!(actual, expected);

        let actual = list.get(2).and_then(|p| p.starting_sector());
        let expected = Some(8192);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_iterate_forwards_over_an_empty_list() -> crate::Result<()> {
        let list = PartitionList::new()?;

        let mut iter = list.iter();
        let actual = iter.next();
        assert!(actual.is_none());

        Ok(())
    }

    #[test]
    fn partition_list_can_iterate_backwards_over_an_empty_list() -> crate::Result<()> {
        let list = PartitionList::new()?;

        let mut iter = list.iter();
        let actual = iter.next_back();
        assert!(actual.is_none());

        Ok(())
    }

    #[test]
    fn partition_list_can_get_the_nth_item() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        let mut iter = list.iter();

        let actual = iter.nth(1).and_then(|p| p.starting_sector());
        let expected = Some(4096);
        assert_eq!(actual, expected);

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_iterate_forwards_over_a_list() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        let mut iter = list.iter();

        let actual = iter.next().and_then(|p| p.starting_sector());
        let expected = Some(64);
        assert_eq!(actual, expected);

        let actual = iter.next().and_then(|p| p.starting_sector());
        let expected = Some(4096);
        assert_eq!(actual, expected);

        let actual = iter.next().and_then(|p| p.starting_sector());
        let expected = Some(8192);
        assert_eq!(actual, expected);

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_iterate_backwards_over_a_list() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        let mut iter = list.iter();

        let actual = iter.next_back().and_then(|p| p.starting_sector());
        let expected = Some(8192);
        assert_eq!(actual, expected);

        let actual = iter.next_back().and_then(|p| p.starting_sector());
        let expected = Some(4096);
        assert_eq!(actual, expected);

        let actual = iter.next_back().and_then(|p| p.starting_sector());
        let expected = Some(64);
        assert_eq!(actual, expected);

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_alternate_forward_backward_iteration() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        let mut iter = list.iter();

        let actual = iter.next().and_then(|p| p.starting_sector());
        let expected = Some(64);
        assert_eq!(actual, expected);

        let actual = iter.next_back().and_then(|p| p.starting_sector());
        let expected = Some(8192);
        assert_eq!(actual, expected);

        let actual = iter.next().and_then(|p| p.starting_sector());
        let expected = Some(4096);
        assert_eq!(actual, expected);

        let actual = iter.next_back();
        assert!(actual.is_none());

        let actual = iter.next();
        assert!(actual.is_none());

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_alternate_backward_forward_iteration() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        let mut iter = list.iter();

        let actual = iter.next_back().and_then(|p| p.starting_sector());
        let expected = Some(8192);
        assert_eq!(actual, expected);

        let actual = iter.next().and_then(|p| p.starting_sector());
        let expected = Some(64);
        assert_eq!(actual, expected);

        let actual = iter.next_back().and_then(|p| p.starting_sector());
        let expected = Some(4096);
        assert_eq!(actual, expected);

        let actual = iter.next();
        assert!(actual.is_none());

        let actual = iter.next_back();
        assert!(actual.is_none());

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_mutably_iterate_forwards_over_an_empty_list() -> crate::Result<()> {
        let list = PartitionList::new()?;

        let mut iter = list.iter_mut();
        let actual = iter.next();
        assert!(actual.is_none());

        Ok(())
    }

    #[test]
    fn partition_list_can_mutably_iterate_backwards_over_an_empty_list() -> crate::Result<()> {
        let list = PartitionList::new()?;

        let mut iter = list.iter_mut();
        let actual = iter.next_back();
        assert!(actual.is_none());

        Ok(())
    }

    #[test]
    fn partition_list_can_mutably_get_the_nth_item() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        // Get the second item mutably
        let mut iter = list.iter_mut();

        let partition = iter.nth(1).unwrap();
        let actual = partition.starting_sector();
        let expected = Some(4096);
        assert_eq!(actual, expected);

        // Mutate second item
        partition.set_starting_sector(1234)?;

        // Get it again immutably
        let mut iter = list.iter();

        let actual = iter.nth(1).and_then(|p| p.starting_sector());
        let expected = Some(1234);
        assert_eq!(actual, expected);

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_mutably_iterate_forwards_over_a_list() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        for partition in list.iter() {
            let actual = partition.size_in_sectors();
            assert!(actual.is_none());
        }

        for partition in list.iter_mut() {
            partition.set_size_in_sectors(128)?;
        }

        for partition in list.iter() {
            let actual = partition.size_in_sectors();
            let expected = Some(128);
            assert_eq!(actual, expected);
        }

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_mutably_iterate_backwards_over_a_list() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        for partition in list.iter() {
            let actual = partition.size_in_sectors();
            assert!(actual.is_none());
        }

        for partition in list.iter_mut().rev() {
            partition.set_size_in_sectors(128)?;
        }

        for partition in list.iter() {
            let actual = partition.size_in_sectors();
            let expected = Some(128);
            assert_eq!(actual, expected);
        }

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn partition_list_can_not_index_into_an_empty_list() {
        let empty = PartitionList::new().unwrap();

        let _ = empty[0];
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn partition_list_can_not_index_out_of_bounds() {
        let partition1 = Partition::builder()
            .number(1)
            .starting_sector(64)
            .build()
            .unwrap();
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()
            .unwrap();
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()
            .unwrap();

        let mut list = PartitionList::new().unwrap();
        list.push(partition1).unwrap();
        list.push(partition2).unwrap();
        list.push(partition3).unwrap();

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        let _ = list[usize::MAX];
    }

    #[test]
    fn partition_list_can_index_into_a_list() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        let actual = list[1].starting_sector();
        let expected = Some(4096);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_list_can_mutably_index_into_a_list() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).starting_sector(64).build()?;
        let partition2 = Partition::builder()
            .number(2)
            .starting_sector(4096)
            .build()?;
        let partition3 = Partition::builder()
            .number(3)
            .starting_sector(8192)
            .build()?;

        let mut list = PartitionList::new()?;
        list.push(partition1)?;
        list.push(partition2)?;
        list.push(partition3)?;

        let actual = list.len();
        let expected = 3;
        assert_eq!(actual, expected);

        let actual = list[1].starting_sector();
        let expected = Some(4096);
        assert_eq!(actual, expected);

        list[1].set_starting_sector(65536)?;

        let actual = list[1].starting_sector();
        let expected = Some(65536);
        assert_eq!(actual, expected);

        Ok(())
    }
}
