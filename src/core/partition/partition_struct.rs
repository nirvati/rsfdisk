// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::cmp::Ordering;
use std::mem::MaybeUninit;

// From this library
use crate::core::errors::PartitionError;

use crate::core::partition::PartBuilder;
use crate::core::partition::PartitionBuilder;
use crate::core::partition::PartitionKind;

use crate::ffi_utils;

/// Partition metadata.
///
/// By default, a `Partition` is set to use the first free partition number available, starting
/// sector, and last free ending sector if respectively a specific partition number, starting
/// sector, and size in sectors are not provided at construction.
///
/// # Examples
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use rsfdisk::core::partition::Partition;
/// use rsfdisk::core::partition::PartitionKind;
/// use rsfdisk::core::partition::Guid;
///
/// fn main() -> rsfdisk::Result<()> {
///     // Default partition
///     let partition = Partition::builder()
///         .build()?;
///
///     // Will use the first partition number available.
///     let actual = partition.number();
///     assert!(actual.is_none());
///
///     // Will use the first free sector.
///     let actual = partition.uses_default_starting_sector();
///     let expected = true;
///     assert_eq!(actual, expected);
///
///     // Will use all available space up to the last free sector.
///     let actual = partition.uses_default_ending_sector();
///     let expected = true;
///     assert_eq!(actual, expected);
///
///     Ok(())
/// }
/// ```
///
/// You can also tailor the characteristics of a `Partition` to suit your needs.
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use rsfdisk::core::partition::Partition;
/// use rsfdisk::core::partition::PartitionKind;
/// use rsfdisk::core::partition::Guid;
///
/// fn main() -> rsfdisk::Result<()> {
///     // Linux Data partition
///     let partition_type = PartitionKind::builder()
///         .guid(Guid::LinuxData)
///         .build()?;
///
///     let partition = Partition::builder()
///         // Set the partition’s type.
///         .partition_type(partition_type)
///         // Set the partition’s name (a human readable label).
///         .name("Linux backup data")
///         // Set the partition’s identification number.
///         .number(1)
///         // Set the offset of the partition’s first sector with respect to the beginning of the
///         // device.
///         .starting_sector(64)
///         // Set the partition’s size in sectors.
///         // Assuming 512 bytes per sector, 16,777,216 sectors <=> 8GiB.
///         .size_in_sectors(16_777_216)
///         .build()?;
///
///     let actual = partition
///         .partition_type()
///         .and_then(|kind| kind.guid().map(String::from));
///     let guid = Guid::LinuxData.to_string();
///     let expected = Some(guid);
///     assert_eq!(actual, expected);
///
///     let actual = partition.name();
///     let name = "Linux backup data";
///     let expected = Some(name);
///     assert_eq!(actual, expected);
///
///     let actual = partition.number();
///     let partition_number = 1;
///     let expected = Some(partition_number);
///     assert_eq!(actual, expected);
///
///     let actual = partition.starting_sector();
///     let start = 64;
///     let expected = Some(start);
///     assert_eq!(actual, expected);
///
///     let actual = partition.size_in_sectors();
///     let size = 16_777_216;
///     let expected = Some(size);
///     assert_eq!(actual, expected);
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Partition {
    pub(crate) inner: *mut libfdisk::fdisk_partition,
}

impl Partition {
    #[doc(hidden)]
    /// Increments the `Partition`'s reference counter.
    pub(crate) fn incr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_ref_partition(self.inner) }
    }

    #[doc(hidden)]
    /// Decrements the `Partition`'s reference counter.
    #[allow(dead_code)]
    pub(crate) fn decr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_unref_partition(self.inner) }
    }

    #[doc(hidden)]
    /// Borrows a `Partition` instance.
    #[allow(dead_code)]
    pub(crate) fn borrow_ptr(ptr: *mut libfdisk::fdisk_partition) -> Partition {
        let mut partition = Self { inner: ptr };
        // We are virtually ceding ownership of this partition which will be automatically
        // deallocated once it is out of scope, incrementing its reference counter protects it from
        // being freed prematurely.
        partition.incr_ref_counter();

        partition
    }

    #[doc(hidden)]
    /// Wraps a boxed raw `libfdisk::fdisk_partition` pointer in a safe reference.
    pub(crate) unsafe fn ref_from_boxed_ptr<'a>(
        ptr: Box<*mut libfdisk::fdisk_partition>,
    ) -> (*mut *mut libfdisk::fdisk_partition, &'a Self) {
        let raw_ptr = Box::into_raw(ptr);
        let entry_ref = unsafe { &*(raw_ptr as *const _ as *const Self) };

        (raw_ptr, entry_ref)
    }

    #[doc(hidden)]
    /// Wraps a boxed raw `libfdisk::fdisk_partition` pointer in a safe reference.
    pub(crate) unsafe fn mut_from_boxed_ptr<'a>(
        ptr: Box<*mut libfdisk::fdisk_partition>,
    ) -> (*mut *mut libfdisk::fdisk_partition, &'a mut Self) {
        let raw_ptr = Box::into_raw(ptr);
        let entry_ref = unsafe { &mut *(raw_ptr as *mut Self) };

        (raw_ptr, entry_ref)
    }

    #[doc(hidden)]
    /// Wraps a raw `libfdisk::fdisk_partition` with a safe `Partition`.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut libfdisk::fdisk_partition) -> Partition {
        Self { inner: ptr }
    }

    #[doc(hidden)]
    /// Creates a new `Partition` instance.
    pub(crate) fn new() -> Result<Partition, PartitionError> {
        log::debug!("Partition::new creating a new `Partition` instance");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_partition>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_new_partition());
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `Partition` instance".to_owned();
                log::debug!(
                    "Partition::new {}. libfdisk::fdisk_new_partition returned a NULL pointer",
                    err_msg
                );

                Err(PartitionError::Creation(err_msg))
            }
            ptr => {
                log::debug!("Partition::new created a new `Partition` instance");
                let partition = Self::from_ptr(ptr);

                Ok(partition)
            }
        }
    }

    #[doc(hidden)]
    /// Sets the partition to ask for a size when used as a template.
    pub(crate) fn ask_size_interactive(&mut self) -> Result<(), PartitionError> {
        log::debug!("Partition::ask_size_interactive setting partition to ask for size");

        let result = unsafe { libfdisk::fdisk_partition_size_explicit(self.inner, 1) };

        match result {
            0 => {
                log::debug!("Partition::ask_size_interactive set partition to ask for size");

                Ok(())
            }
            code => {
                let err_msg = "failed to set partition to ask for size".to_owned();
                log::debug!("Partition::ask_size_interactive {}. libfdisk::fdisk_partition_size_explicit returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Sets attribute bits to define partition usage.
    pub(crate) fn set_attribute_bits(&mut self, mut bits: Vec<u8>) -> Result<(), PartitionError> {
        log::debug!("Partition::set_attribute_bits setting attribute bits");
        // Add NULL terminal character to char array.
        bits.push(0);

        let result =
            unsafe { libfdisk::fdisk_partition_set_attrs(self.inner, bits.as_ptr() as *const _) };

        match result {
            0 => {
                log::debug!("Partition::set_attribute_bits set attribute bits");

                Ok(())
            }
            code => {
                let err_msg = "failed to set attribute bits".to_owned();
                log::debug!("Partition::set_attribute_bits {}. libfdisk::fdisk_partition_set_attrs returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Sets this `Partition`'s name.
    pub(crate) fn set_name(&mut self, name: String) -> Result<(), PartitionError> {
        log::debug!("Partition::set_name setting partition name to: {:?}", name);
        let name_cstr = ffi_utils::as_ref_str_to_c_string(&name)?;

        let result = unsafe { libfdisk::fdisk_partition_set_name(self.inner, name_cstr.as_ptr()) };

        match result {
            0 => {
                log::debug!("Partition::set_name set partition name to: {:?}", name);

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set partition name to: {:?}", name);
                log::debug!("Partition::set_name {}. libfdisk::fdisk_parttype_set_name returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Sets this `Partition`'s UUID.
    pub(crate) fn set_uuid(&mut self, uuid: String) -> Result<(), PartitionError> {
        let uuid_cstr = ffi_utils::as_ref_str_to_c_string(&uuid)?;
        log::debug!("Partition::set_uuid setting partition UUID to: {:?}", uuid);

        let result = unsafe { libfdisk::fdisk_partition_set_uuid(self.inner, uuid_cstr.as_ptr()) };

        match result {
            0 => {
                log::debug!("Partition::set_uuid partition UUID set to: {:?}", uuid);

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set partition UUID to: {:?}", uuid);
                log::debug!("Partition::set_uuid {}. libfdisk::fdisk_partition_set_uuid returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Set the partition's identification number to the first available. Set to `true` by default.
    pub(crate) fn use_first_free_partition_number(
        &mut self,
        enable: bool,
    ) -> Result<(), PartitionError> {
        let op_str = if enable {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        let op = if enable { 1 } else { 0 };
        log::debug!("Partition::use_first_free_partition_number setting partition to use first free partition number to: {:?}", op_str);

        let result = unsafe { libfdisk::fdisk_partition_partno_follow_default(self.inner, op) };

        match result {
            0 => {
                log::debug!("Partition::use_first_free_partition_number {}d partition to use first free partition number", op_str);

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to {} partition to use first free partition number",
                    op_str
                );
                log::debug!("Partition::use_first_free_partition_number {}. libfdisk::fdisk_partition_partno_follow_default returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Set the partition's starting sector to the first unoccupied sector available. Set to `true` by default.
    pub(crate) fn use_first_free_starting_sector(
        &mut self,
        enable: bool,
    ) -> Result<(), PartitionError> {
        let op_str = if enable {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        let op = if enable { 1 } else { 0 };
        log::debug!("Partition::use_first_free_starting_sector setting partition to use first available free sector to: {:?}", op_str);

        let result = unsafe { libfdisk::fdisk_partition_start_follow_default(self.inner, op) };

        match result {
            0 => {
                log::debug!("Partition::use_first_free_starting_sector {}d partition to use first available free sector as starting sector", op_str);

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to {} partition to use first available free sector as starting sector",
                    op_str
                );
                log::debug!("Partition::use_first_free_starting_sector {}. libfdisk::fdisk_partition_start_follow_default returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Set the partition's ending sector to the last unoccupied sector available (forces the partition to use all available free space). Set to `true` by default.
    pub(crate) fn use_last_free_ending_sector(
        &mut self,
        enable: bool,
    ) -> Result<(), PartitionError> {
        let op_str = if enable {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        let op = if enable { 1 } else { 0 };
        log::debug!("Partition::use_last_free_ending_sector setting partition to use all free space for partition: {:?}", op_str);

        let result = unsafe { libfdisk::fdisk_partition_end_follow_default(self.inner, op) };

        match result {
            0 => {
                log::debug!("Partition::use_last_free_ending_sector {}d partition to use all free space for partition", op_str);

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to {} partition to use all free space for partition",
                    op_str
                );
                log::debug!("Partition::use_last_free_ending_sector {}. libfdisk::fdisk_partition_end_follow_default returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    /// Creates a [`PartitionBuilder`] to configure and construct a new `Partition` instance.
    ///
    /// Call the [`PartitionBuilder`]'s [`build()`](crate::core::partition::PartitionBuilder::build) method to
    /// instantiate a new `Partition`.
    pub fn builder() -> PartitionBuilder {
        log::debug!("Partition::builder creating a new `PartitionBuilder` instance");

        PartBuilder::builder()
    }

    //---- BEGIN getters

    /// Returns this `Partition`'s attribute bits.
    pub fn attribute_bits(&self) -> Option<Vec<u8>> {
        log::debug!("Partition::attribute_bits getting partition attribute bits");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_partition_get_attrs(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to get partition attribute bits".to_owned();
                log::debug!("Partition::attribute_bits {}. libfdisk::fdisk_partition_get_attrs returned a NULL pointer", err_msg);

                None
            }
            attrs_ptr => {
                log::debug!("Partition::attribute_bits got partition attribute bits");
                let attrs = ffi_utils::const_c_char_array_to_bytes(attrs_ptr);

                Some(attrs.to_owned())
            }
        }
    }

    /// Returns the address of this `Partition`'s first sector, or `None` if it is not set.
    pub fn starting_sector(&self) -> Option<u64> {
        if self.has_set_starting_sector() {
            let first_sector = unsafe { libfdisk::fdisk_partition_get_start(self.inner) };

            log::debug!(
                "Partition::starting_sector first partition sector: {:?}",
                first_sector
            );

            Some(first_sector)
        } else {
            log::debug!("Partition::starting_sector value not set");

            None
        }
    }

    /// Returns the address of this `Partition`'s last sector, or `None` if it is not set.
    pub fn ending_sector(&self) -> Option<u64> {
        if self.has_set_ending_sector() {
            let last_sector = unsafe { libfdisk::fdisk_partition_get_end(self.inner) };

            log::debug!(
                "Partition::ending_sector last partition sector: {:?}",
                last_sector
            );

            Some(last_sector)
        } else {
            log::debug!("Partition::ending_sector last partition sector not set");

            None
        }
    }

    /// Returns this `Partition`'s name.
    pub fn name(&self) -> Option<&str> {
        log::debug!("Partition::name getting partition name");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_partition_get_name(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("Partition::name got no partition name. libfdisk::fdisk_parttype_get_name returned a NULL pointer");

                None
            }
            name_ptr => {
                let name = ffi_utils::const_char_array_to_str_ref(name_ptr).ok();
                log::debug!("Partition::name partition name: {:?}", name);

                name
            }
        }
    }

    /// Returns this `Partition`'s identification number, or `None` if it is not set.
    ///
    /// **Note:** `0` is a valid partition identification number.
    pub fn number(&self) -> Option<usize> {
        if self.has_set_partition_number() {
            let number = unsafe { libfdisk::fdisk_partition_get_partno(self.inner) };

            log::debug!(
                "Partition::number partition identification number: {:?}",
                number
            );

            Some(number)
        } else {
            log::debug!("Partition::number partition identification number not set");

            None
        }
    }

    /// Returns the identification number of this `Partition`'s parent partition.
    pub fn parent_partition_number(&self) -> Option<usize> {
        log::debug!(
            "Partition::parent_partition_number getting parent partition's identification number"
        );

        let mut ptr = MaybeUninit::<libc::size_t>::zeroed();

        let result = unsafe { libfdisk::fdisk_partition_get_parent(self.inner, ptr.as_mut_ptr()) };

        match result {
            0 => {
                let number = unsafe { ptr.assume_init() };
                log::debug!("Partition::parent_partition_number got parent partition's identification number: {:?}", number);

                Some(number)
            }
            code => {
                log::debug!("Partition::parent_partition_number failed to get parent partition's identification number. libfdisk::fdisk_partition_get_parent returned error code: {:?}", code);

                None
            }
        }
    }

    /// Returns a reference to this `Partition`'s type, or `None` if it is not set.
    pub fn partition_type(&self) -> Option<PartitionKind> {
        log::debug!("Partition::partition_type getting a reference to the partition's type");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_parttype>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_partition_get_type(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to get partition type".to_owned();
                log::debug!("Partition::partition_type {}. libfdisk::fdisk_partition_get_type returned a NULL pointer", err_msg);

                None
            }
            type_ptr => {
                log::debug!("Partition::partition_type got the partition's type");
                let kind = PartitionKind::borrow_ptr(type_ptr);

                Some(kind)
            }
        }
    }

    /// Returns this `Partition`'s size in sectors, or `None` if it is not set.
    pub fn size_in_sectors(&self) -> Option<u64> {
        if self.has_set_size() {
            let size = unsafe { libfdisk::fdisk_partition_get_size(self.inner) };

            log::debug!("Partition::size_in_sectors size: {:?}", size);

            Some(size)
        } else {
            log::debug!("Partition::size_in_sectors partition size not set");

            None
        }
    }

    /// Returns this `Partition`'s UUID, or `None` if it is not set.
    pub fn uuid(&self) -> Option<&str> {
        log::debug!("Partition::uuid getting partition UUID");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_partition_get_uuid(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("Partition::uuid got no partition UUID. libfdisk::fdisk_partition_get_uuid returned a NULL pointer");

                None
            }
            uuid_ptr => {
                let uuid = ffi_utils::const_char_array_to_str_ref(uuid_ptr).ok();
                log::debug!("Partition::uuid got partition UUID: {:?}", uuid);

                uuid
            }
        }
    }

    //---- END getters

    /// Compares the values of `Partition`s identification numbers.
    pub fn compare_partition_numbers(&self, other: &Partition) -> Ordering {
        log::debug!("Partition::compare_partition_numbers comparing partition numbers");

        let result = unsafe { libfdisk::fdisk_partition_cmp_partno(self.inner, other.inner) };

        match result {
            cmp if cmp < 0 => {
                let ordering = Ordering::Less;
                log::debug!("Partition::compare_partition_numbers value: {:?}", ordering);

                ordering
            }
            0 => {
                let ordering = Ordering::Equal;
                log::debug!("Partition::compare_partition_numbers value: {:?}", ordering);

                ordering
            }
            _otherwise => {
                let ordering = Ordering::Greater;
                log::debug!("Partition::compare_partition_numbers value: {:?}", ordering);

                ordering
            }
        }
    }

    /// Compares the values of `Partition`s starting sectors.
    pub fn compare_starting_sectors(&self, other: &Partition) -> Ordering {
        log::debug!("Partition::compare_starting_sectors comparing partition starting sectors");

        let result = unsafe { libfdisk::fdisk_partition_cmp_start(self.inner, other.inner) };

        match result {
            cmp if cmp < 0 => {
                let ordering = Ordering::Less;
                log::debug!("Partition::compare_starting_sectors value: {:?}", ordering);

                ordering
            }
            0 => {
                let ordering = Ordering::Equal;
                log::debug!("Partition::compare_starting_sectors value: {:?}", ordering);

                ordering
            }
            _otherwise => {
                let ordering = Ordering::Greater;
                log::debug!("Partition::compare_starting_sectors value: {:?}", ordering);

                ordering
            }
        }
    }

    //---- BEGIN setters

    #[doc(hidden)]
    /// Sets/Unsets a partition's type.
    fn set_type(
        partition: &mut Self,
        parttype: *mut libfdisk::fdisk_parttype,
    ) -> Result<(), PartitionError> {
        let result = unsafe { libfdisk::fdisk_partition_set_type(partition.inner, parttype) };

        match result {
            0 => {
                log::debug!("Partition::set_type partition type set");

                Ok(())
            }
            code => {
                let err_msg = "failed to set partition type".to_owned();
                log::debug!("Partition::set_type {}. libfdisk::fdisk_partition_set_type returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    /// Sets the `Partition`'s type.
    pub fn set_partition_type(&mut self, kind: PartitionKind) -> Result<(), PartitionError> {
        log::debug!("Partition::set_partition_type setting partition type");

        Self::set_type(self, kind.inner)
    }

    // /// Sets this `Partition`'s type as `undefined`.
    // pub fn unset_partition_type(&mut self) -> Result<(), PartitionError> {
    //     log::debug!("Partition::unset_partition_type unsetting partition type");

    //     Self::set_type(self, std::ptr::null_mut())
    // }

    /// Sets this `Partition`'s identification number.
    pub fn set_partition_number(&mut self, number: usize) -> Result<(), PartitionError> {
        log::debug!(
            "Partition::set_partition_number setting partition number to: {:?}",
            number
        );

        self.use_first_free_partition_number(false)?;

        let result = unsafe { libfdisk::fdisk_partition_set_partno(self.inner, number) };

        match result {
            0 => {
                log::debug!(
                    "Partition::set_partition_number set partition number to: {:?}",
                    number
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set partition number to: {:?}", number);
                log::debug!("Partition::set_partition_number {}. libfdisk::fdisk_partition_set_partno returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    /// Removes this `Partition`'s identification number, and sets it to use the first free
    /// partition number.
    pub fn unset_partition_number(&mut self) -> Result<(), PartitionError> {
        log::debug!("Partition::unset_partition_number unsetting partition number");

        self.use_first_free_partition_number(true)?;

        let result = unsafe { libfdisk::fdisk_partition_unset_partno(self.inner) };

        match result {
            0 => {
                log::debug!("Partition::unset_partition_number partition number unset");

                Ok(())
            }
            code => {
                let err_msg = "failed to unset partition number".to_owned();
                log::debug!("Partition::unset_partition_number {}. libfdisk::fdisk_partition_unset_partno returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    /// Sets this `Partition`'s size in sectors.
    pub fn set_size_in_sectors(&mut self, size: u64) -> Result<(), PartitionError> {
        log::debug!(
            "Partition::set_size_in_sectors setting partition size to (sectors): {:?}",
            size
        );

        self.use_last_free_ending_sector(false)?;

        let result = unsafe { libfdisk::fdisk_partition_set_size(self.inner, size) };

        match result {
            0 => {
                log::debug!(
                    "Partition::set_size_in_sectors partition size set to (sectors): {:?}",
                    size
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set partition size to (sectors): {:?}", size);
                log::debug!("Partition::set_size_in_sectors {}. libfdisk::fdisk_partition_set_size returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    /// Removes this `Partition`'s size in sectors, and sets it to use the last free
    /// ending sector.
    pub fn unset_size_in_sectors(&mut self) -> Result<(), PartitionError> {
        log::debug!("Partition::unset_size_in_sectors unsetting partition size");

        self.use_last_free_ending_sector(true)?;

        let result = unsafe { libfdisk::fdisk_partition_unset_size(self.inner) };

        match result {
            0 => {
                log::debug!("Partition::unset_size_in_sectors partition size unset");

                Ok(())
            }
            code => {
                let err_msg = "failed to unset partition size".to_owned();
                log::debug!("Partition::unset_size_in_sectors {}. libfdisk::fdisk_partition_unset_size returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    /// Sets the address of this `Partition`'s first sector (i.e. offset with respect to the beginning of the device).
    pub fn set_starting_sector(&mut self, address: u64) -> Result<(), PartitionError> {
        log::debug!(
            "Partition::set_starting_sector setting partition's starting sector at: {:?}",
            address
        );

        self.use_first_free_starting_sector(false)?;

        let result = unsafe { libfdisk::fdisk_partition_set_start(self.inner, address) };

        match result {
            0 => {
                log::debug!(
                    "Partition::set_starting_sector partition's starting sector set to: {:?}",
                    address
                );

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to set partition's starting sector to: {:?}",
                    address
                );
                log::debug!("Partition::set_starting_sector {}. libfdisk::fdisk_partition_set_start returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    /// Removes this `Partition`'s starting sector, and sets it to use the first free
    /// starting sector.
    pub fn unset_starting_sector(&mut self) -> Result<(), PartitionError> {
        log::debug!("Partition::unset_starting_sector unsetting partition first sector");

        self.use_first_free_starting_sector(true)?;

        let result = unsafe { libfdisk::fdisk_partition_unset_start(self.inner) };

        match result {
            0 => {
                log::debug!("Partition::unset_starting_sector partition first sector unset");

                Ok(())
            }
            code => {
                let err_msg = "failed to unset partition first sector".to_owned();
                log::debug!("Partition::unset_starting_sector {}. libfdisk::fdisk_partition_unset_start returned error code: {:?}", err_msg, code);

                Err(PartitionError::Config(err_msg))
            }
        }
    }

    //---- END setters

    //--- BEGIN predicates

    /// Returns `true` if the value of this `Partition`'s first sector is set.
    fn has_set_starting_sector(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_has_start(self.inner) == 1 };
        log::debug!("Partition::has_set_starting_sector value: {:?}", state);

        state
    }

    /// Returns `true` if the value of this `Partition`'s last sector is set.
    fn has_set_ending_sector(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_has_end(self.inner) == 1 };
        log::debug!("Partition::has_set_ending_sector value: {:?}", state);

        state
    }

    /// Returns `true` if the value of this `Partition`'s identification is set.
    fn has_set_partition_number(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_has_partno(self.inner) == 1 };
        log::debug!("Partition::has_set_partition_number value: {:?}", state);

        state
    }

    /// Returns `true` if the value of this `Partition`'s size is set.
    fn has_set_size(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_has_size(self.inner) == 1 };
        log::debug!("Partition::has_set_size value: {:?}", state);

        state
    }

    /// Returns `true` if this `Partition`'s "bootable" attribute bit is set.
    pub fn is_bootable(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_is_bootable(self.inner) == 1 };
        log::debug!("Partition::is_bootable value: {:?}", state);

        state
    }

    /// Returns `true` if this `Partition` is a container (e.g. a `MBR Extended` partition).
    pub fn is_container(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_is_container(self.inner) == 1 };
        log::debug!("Partition::is_container value: {:?}", state);

        state
    }

    /// Returns `true` if this `Partition` points to free space on a device.
    pub fn is_free_space(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_is_freespace(self.inner) == 1 };
        log::debug!("Partition::is_free_space value: {:?}", state);

        state
    }

    /// Returns `true` if this `Partition` is a nested partition.
    pub fn is_nested(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_is_nested(self.inner) == 1 };
        log::debug!("Partition::is_nested value: {:?}", state);

        state
    }

    /// Returns `true` if this `Partition` is a special "whole-disk" partition (e.g. `SUN` partition).
    pub fn is_whole_disk(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_is_wholedisk(self.inner) == 1 };
        log::debug!("Partition::is_whole_disk value: {:?}", state);

        state
    }

    /// Returns `true` if this `Partition` points to an area in use.
    pub fn points_to_used_area(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_is_used(self.inner) == 1 };
        log::debug!("Partition::is_used value: {:?}", state);

        state
    }

    /// Returns `true` if this `Partition`'s ending sector is set to use all free space.
    pub fn uses_default_ending_sector(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_end_is_default(self.inner) == 1 };
        log::debug!("Partition::uses_default_ending_sector value: {:?}", state);

        state
    }

    /// Returns `true` if this `Partition`'s starting sector is set to use the first free sector.
    pub fn uses_default_starting_sector(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_start_is_default(self.inner) == 1 };
        log::debug!("Partition::uses_default_starting_sector value: {:?}", state);

        state
    }

    //--- END predicates
}

impl AsRef<Partition> for Partition {
    #[inline]
    fn as_ref(&self) -> &Partition {
        self
    }
}

impl Drop for Partition {
    fn drop(&mut self) {
        log::debug!("Partition::drop deallocating `Partition` instance");

        unsafe { libfdisk::fdisk_unref_partition(self.inner) }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::core::partition::Guid;
    use crate::core::partition::PartitionKind;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::cmp::Ordering;

    #[test]
    fn partition_can_create_a_new_partition() -> crate::Result<()> {
        // Linux partition
        let guid = Guid::LinuxData;
        let partition_kind = PartitionKind::builder().guid(guid).build()?;

        let name = "Linux backup data";
        let number = 1;
        let size = 204800;
        let partition = Partition::builder()
            .partition_type(partition_kind)
            .name(name)
            .number(number)
            // Assuming 512 bytes per sector, 204800 sectors <=> 100MiB.
            .size_in_sectors(size)
            .build()?;

        let actual = partition
            .partition_type()
            .and_then(|kind| kind.guid().map(String::from));
        let guid_string = guid.to_string();
        let expected = Some(guid_string);
        assert_eq!(actual, expected);

        let actual = partition.name();
        let expected = Some(name);
        assert_eq!(actual, expected);

        let actual = partition.number();
        let expected = Some(number);
        assert_eq!(actual, expected);

        let actual = partition.size_in_sectors();
        let expected = Some(size);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_can_compare_partition_numbers() -> crate::Result<()> {
        let partition1 = Partition::builder().number(1).build()?;
        let partition2 = Partition::builder().number(2).build()?;

        let actual = partition1.compare_partition_numbers(&partition1);
        let expected = Ordering::Equal;
        assert_eq!(actual, expected);

        let actual = partition1.compare_partition_numbers(&partition2);
        let expected = Ordering::Less;
        assert_eq!(actual, expected);

        let actual = partition2.compare_partition_numbers(&partition1);
        let expected = Ordering::Greater;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_can_compare_starting_sectors() -> crate::Result<()> {
        let partition1 = Partition::builder().starting_sector(64).build()?;
        let partition2 = Partition::builder().starting_sector(128).build()?;

        let actual = partition1.compare_starting_sectors(&partition1);
        let expected = Ordering::Equal;
        assert_eq!(actual, expected);

        let actual = partition1.compare_starting_sectors(&partition2);
        let expected = Ordering::Less;
        assert_eq!(actual, expected);

        let actual = partition2.compare_starting_sectors(&partition1);
        let expected = Ordering::Greater;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_can_set_a_partition_type() -> crate::Result<()> {
        let mut partition = Partition::builder().build()?;

        let actual = partition.partition_type();
        assert!(actual.is_none());

        let guid = Guid::SolarisRoot;
        let partition_kind = PartitionKind::builder().guid(guid).build()?;
        partition.set_partition_type(partition_kind)?;

        let actual = partition
            .partition_type()
            .and_then(|kind| kind.guid().map(String::from));
        let guid_string = guid.to_string();
        let expected = Some(guid_string);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_can_set_a_partition_number() -> crate::Result<()> {
        let mut partition = Partition::builder().build()?;

        let actual = partition.number();
        let expected = None;
        assert_eq!(actual, expected);

        let number = 1;
        partition.set_partition_number(number)?;

        let actual = partition.has_set_partition_number();
        let expected = true;
        assert_eq!(actual, expected);

        let actual = partition.number();
        let expected = Some(number);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_can_unset_a_partition_number() -> crate::Result<()> {
        let number = 1;
        let mut partition = Partition::builder().number(number).build()?;

        let actual = partition.number();
        let expected = Some(number);
        assert_eq!(actual, expected);

        partition.unset_partition_number()?;

        let actual = partition.has_set_partition_number();
        let expected = false;
        assert_eq!(actual, expected);

        let actual = partition.number();
        let expected = None;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_can_set_a_partition_size_in_sectors() -> crate::Result<()> {
        let mut partition = Partition::builder().build()?;

        let actual = partition.size_in_sectors();
        let expected = None;
        assert_eq!(actual, expected);

        let size = 1024;
        partition.set_size_in_sectors(size)?;

        let actual = partition.has_set_size();
        let expected = true;
        assert_eq!(actual, expected);

        let actual = partition.uses_default_ending_sector();
        let expected = false;
        assert_eq!(actual, expected);

        let actual = partition.size_in_sectors();
        let expected = Some(1024);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_can_unset_a_partition_size_in_sectors() -> crate::Result<()> {
        let size = 1024;
        let mut partition = Partition::builder().size_in_sectors(size).build()?;

        let actual = partition.size_in_sectors();
        let expected = Some(size);
        assert_eq!(actual, expected);

        partition.unset_size_in_sectors()?;

        let actual = partition.has_set_size();
        let expected = false;
        assert_eq!(actual, expected);

        let actual = partition.uses_default_ending_sector();
        let expected = true;
        assert_eq!(actual, expected);

        let actual = partition.size_in_sectors();
        let expected = None;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_can_set_a_partition_starting_sector() -> crate::Result<()> {
        let mut partition = Partition::builder().build()?;

        let actual = partition.starting_sector();
        let expected = None;
        assert_eq!(actual, expected);

        let start = 64;
        partition.set_starting_sector(start)?;

        let actual = partition.has_set_starting_sector();
        let expected = true;
        assert_eq!(actual, expected);

        let actual = partition.uses_default_starting_sector();
        let expected = false;
        assert_eq!(actual, expected);

        let actual = partition.starting_sector();
        let expected = Some(start);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_can_unset_a_partition_starting_sector() -> crate::Result<()> {
        let start = 64;
        let mut partition = Partition::builder().starting_sector(start).build()?;

        let actual = partition.starting_sector();
        let expected = Some(start);
        assert_eq!(actual, expected);

        partition.unset_starting_sector()?;

        let actual = partition.has_set_starting_sector();
        let expected = false;
        assert_eq!(actual, expected);

        let actual = partition.uses_default_starting_sector();
        let expected = true;
        assert_eq!(actual, expected);

        let actual = partition.starting_sector();
        let expected = None;
        assert_eq!(actual, expected);

        Ok(())
    }
}
