// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::errors::PartitionIterError;

use crate::core::partition::Partition;
use crate::core::partition::PartitionList;

use crate::core::iter::Direction;
use crate::core::iter::GenIterator;

use crate::owning_mut_from_ptr;

/// Iterator over [`Partition`]s in a [`PartitionList`].
#[derive(Debug)]
pub struct PartitionIterMut<'vec> {
    list: &'vec PartitionList,
    /// Forward iterator.
    fwd_iter: GenIterator,
    /// Backward iterator.
    bwd_iter: GenIterator,
    /// Current item in forward iteration.
    fwd_cursor: *mut libfdisk::fdisk_partition,
    /// Current item in backward iteration.
    bwd_cursor: *mut libfdisk::fdisk_partition,
    /// Indicator of forward and backward iterators meeting in the middle.
    have_iterators_met: bool,
}

impl<'vec> PartitionIterMut<'vec> {
    #[doc(hidden)]
    #[allow(dead_code)]
    /// Creates a new `PartitionIterMut`.
    pub(crate) fn new(
        list: &'vec PartitionList,
    ) -> Result<PartitionIterMut<'vec>, PartitionIterError> {
        log::debug!("PartitionIterMut::new creating a new `PartitionIterMut` instance");

        let fwd_iter = GenIterator::new(Direction::Forward)?;
        let bwd_iter = GenIterator::new(Direction::Backward)?;
        let fwd_cursor = std::ptr::null_mut();
        let bwd_cursor = std::ptr::null_mut();
        let have_iterators_met = false;

        let iterator = Self {
            list,
            fwd_iter,
            bwd_iter,
            fwd_cursor,
            bwd_cursor,
            have_iterators_met,
        };

        Ok(iterator)
    }
}

impl<'vec> Iterator for PartitionIterMut<'vec> {
    type Item = &'vec mut Partition;

    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("PartitionIterMut::next getting next item in `PartitionList`");

        let mut partition_ptr = MaybeUninit::<*mut libfdisk::fdisk_partition>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_table_next_partition(
                self.list.inner,
                self.fwd_iter.inner,
                partition_ptr.as_mut_ptr(),
            )
        };

        match result {
            0 => {
                let ptr = unsafe { partition_ptr.assume_init() };

                // Per the documentation of `DoubleEndedIterator`
                // "It is important to note that both back and forth work on the same range, and do not cross: iteration is over when they meet in the middle."
                if self.have_iterators_met
                    || (self.fwd_cursor != self.bwd_cursor && ptr == self.bwd_cursor)
                {
                    log::debug!(
                        "PartitionIterMut::next forward and backward iterators met in the middle"
                    );
                    self.have_iterators_met = true;

                    None
                } else {
                    log::debug!("PartitionIterMut::next got next item in `PartitionList`");
                    self.fwd_cursor = ptr;
                    let partition = owning_mut_from_ptr!(self.list, Partition, ptr);

                    Some(partition)
                }
            }
            // Reached end of list.
            1 => {
                log::debug!("PartitionIterMut::next reached end of `PartitionList`");

                None
            }
            // Error occurred.
            code => {
                log::debug!("PartitionIterMut::next failed to get next item in `PartitionList`. libfdisk::fdisk_table_next_partition returned error code: {:?}", code);

                None
            }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let mut result;

        // Skip n-1 entries, and update the cursor along the way.
        for i in 0..n {
            let mut partition_ptr = MaybeUninit::<*mut libfdisk::fdisk_partition>::zeroed();

            result = unsafe {
                libfdisk::fdisk_table_next_partition(
                    self.list.inner,
                    self.fwd_iter.inner,
                    partition_ptr.as_mut_ptr(),
                )
            };

            match result {
                0 => {
                    let ptr = unsafe { partition_ptr.assume_init() };

                    // Per the documentation of `DoubleEndedIterator`
                    // "It is important to note that both back and forth work on the same range, and do not cross: iteration is over when they meet in the middle."
                    if self.have_iterators_met
                        || (self.fwd_cursor != self.bwd_cursor && ptr == self.bwd_cursor)
                    {
                        log::debug!(
                        "PartitionIterMut::nth forward and backward iterators met in the middle"
                    );
                        self.have_iterators_met = true;

                        return None;
                    } else {
                        log::debug!("PartitionIterMut::nth got {}th item in `PartitionList`", i);
                        self.fwd_cursor = ptr;
                    }
                }
                // Reached end of list.
                1 => {
                    log::debug!("PartitionIterMut::nth reached end of `PartitionList`");

                    return None;
                }
                // Error occurred.
                code => {
                    let err_msg = format!("failed to get {}th item in `PartitionList`", i);
                    log::debug!("PartitionIterMut::nth {}. libfdisk::fdisk_table_next_partition returned error code: {:?}", err_msg, code);

                    return None;
                }
            }
        }
        self.next()
    }
}

impl<'vec> DoubleEndedIterator for PartitionIterMut<'vec> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut partition_ptr = MaybeUninit::<*mut libfdisk::fdisk_partition>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_table_next_partition(
                self.list.inner,
                self.bwd_iter.inner,
                partition_ptr.as_mut_ptr(),
            )
        };

        match result {
            0 => {
                let ptr = unsafe { partition_ptr.assume_init() };

                // Per the documentation of `DoubleEndedIterator`
                // "It is important to note that both back and forth work on the same range, and do not cross: iteration is over when they meet in the middle."
                if self.have_iterators_met
                    || (self.bwd_cursor != self.fwd_cursor && ptr == self.fwd_cursor)
                {
                    log::debug!(
                        "PartitionIterMut::next forward and backward iterators met in the middle"
                    );
                    self.have_iterators_met = true;

                    None
                } else {
                    log::debug!("PartitionIterMut::next got next item in `PartitionList`");
                    self.bwd_cursor = ptr;
                    let partition = owning_mut_from_ptr!(self.list, Partition, ptr);

                    Some(partition)
                }
            }
            // Reached end of list.
            1 => {
                log::debug!("PartitionIterMut::next_back reached start of `PartitionList`");

                None
            }
            // Error occurred.
            code => {
                log::debug!("PartitionIterMut::next_back failed to get next item in `PartitionList`. libfdisk::fdisk_table_next_partition returned error code: {:?}", code);

                None
            }
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let mut result;

        // Skip n-1 entries, and update the cursor along the way.
        for i in 0..n {
            let mut partition_ptr = MaybeUninit::<*mut libfdisk::fdisk_partition>::zeroed();

            result = unsafe {
                libfdisk::fdisk_table_next_partition(
                    self.list.inner,
                    self.bwd_iter.inner,
                    partition_ptr.as_mut_ptr(),
                )
            };
            match result {
                0 => {
                    let ptr = unsafe { partition_ptr.assume_init() };

                    // Per the documentation of `DoubleEndedIterator`
                    // "It is important to note that both back and forth work on the same range, and do not cross: iteration is over when they meet in the middle."
                    if self.have_iterators_met
                        || (self.bwd_cursor != self.fwd_cursor && ptr == self.fwd_cursor)
                    {
                        log::debug!(
                        "PartitionIterMut::nth_back forward and backward iterators met in the middle"
                    );
                        self.have_iterators_met = true;

                        return None;
                    } else {
                        log::debug!(
                            "PartitionIterMut::nth_back got {}th item in `PartitionList`",
                            i
                        );
                        self.bwd_cursor = ptr;
                    }
                }
                // Reached end of list.
                1 => {
                    log::debug!("PartitionIterMut::nth_back reached end of `PartitionList`");

                    return None;
                }
                // Error occurred.
                code => {
                    let err_msg = format!("failed to get {}th item in `PartitionList`", i);
                    log::debug!("PartitionIterMut::nth_back {}. libfdisk::fdisk_table_next_partition returned error code: {:?}", err_msg, code);

                    return None;
                }
            }
        }
        self.next_back()
    }
}
