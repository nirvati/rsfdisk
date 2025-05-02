// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::errors::PartitionKindError;
use crate::core::partition::Code;
use crate::core::partition::Guid;
use crate::core::partition::PartTypeBuilder;
use crate::core::partition::PartitionKindBuilder;

use crate::ffi_utils;

/// Type of partition.
///
/// There are two families of partitions GUID/GPT-based amd MBR-based. GUID/GPT partitions are
/// identified by a [16-byte
/// PartitionTypeGUID](https://uefi.org/specs/UEFI/2.10/05_GUID_Partition_Table_Format.html#gpt-partition-entry-array),
/// while MBR partitions are by a [1-byte hexadecimal code
/// (OSType)](https://uefi.org/specs/UEFI/2.10/05_GUID_Partition_Table_Format.html#legacy-master-boot-record-mbr).
#[derive(Debug)]
#[repr(transparent)]
pub struct PartitionKind {
    pub(crate) inner: *mut libfdisk::fdisk_parttype,
}

impl PartitionKind {
    #[doc(hidden)]
    /// Increments the `PartitionKind`'s reference counter.
    pub(crate) fn incr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_ref_parttype(self.inner) }
    }

    #[doc(hidden)]
    /// Decrements the `PartitionKind`'s reference counter.
    #[allow(dead_code)]
    pub(crate) fn decr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_unref_parttype(self.inner) }
    }

    #[doc(hidden)]
    /// Borrows a `PartitionKind` instance.
    #[allow(dead_code)]
    pub(crate) fn borrow_ptr(ptr: *mut libfdisk::fdisk_parttype) -> PartitionKind {
        let mut kind = Self { inner: ptr };
        // We are virtually ceding ownership of this instance which will be automatically
        // deallocated once it is out of scope, incrementing its reference counter protects it from
        // being freed prematurely.
        kind.incr_ref_counter();

        kind
    }

    #[doc(hidden)]
    /// Wraps a raw `libfdisk::fdisk_parttype` with a safe `PartitionKind`.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(ptr: *mut libfdisk::fdisk_parttype) -> PartitionKind {
        Self { inner: ptr }
    }

    #[doc(hidden)]
    /// Creates a new `PartitionKind` instance.
    pub(crate) fn new() -> Result<PartitionKind, PartitionKindError> {
        log::debug!("PartitionKind::new creating a new `PartitionKind` instance");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_parttype>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_new_parttype());
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `PartitionKind` instance".to_owned();
                log::debug!(
                    "PartitionKind::new {}. libfdisk::fdisk_new_parttype returned a NULL pointer",
                    err_msg
                );

                Err(PartitionKindError::Creation(err_msg))
            }
            ptr => {
                log::debug!("PartitionKind::new created a new `PartitionKind` instance");

                let partition_kind = Self::from_ptr(ptr);

                Ok(partition_kind)
            }
        }
    }

    #[doc(hidden)]
    /// Creates a new unknown `PartitionKind` instance.
    pub(crate) fn new_unkown<T>(code: u32, kind: T) -> Result<PartitionKind, PartitionKindError>
    where
        T: AsRef<str>,
    {
        log::debug!("PartitionKind::new_unkown creating a new `PartitionKind` instance");
        let kind = kind.as_ref();
        let kind_cstr = ffi_utils::as_ref_str_to_c_string(kind).map_err(|e| {
            let err_msg = format!("failed to convert value to `CString` {e}");
            PartitionKindError::CStringConversion(err_msg)
        })?;

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_parttype>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_new_unknown_parttype(
                code,
                kind_cstr.as_ptr(),
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `PartitionKind` instance".to_owned();
                log::debug!("PartitionKind::new_unkown {}. libfdisk::fdisk_new_unknown_parttype returned a NULL pointer", err_msg);

                Err(PartitionKindError::Creation(err_msg))
            }
            ptr => {
                log::debug!("PartitionKind::new_unkown created a new `PartitionKind` instance");

                let partition_kind = Self::from_ptr(ptr);

                Ok(partition_kind)
            }
        }
    }

    #[doc(hidden)]
    /// Sets a partition type's identification code.
    pub(crate) fn set_code(&mut self, code: Code) -> Result<(), PartitionKindError> {
        log::debug!(
            "PartitionKind::set_code setting partition type code to: {}",
            code
        );

        let result = unsafe { libfdisk::fdisk_parttype_set_code(self.inner, code as u8 as i32) };

        match result {
            0 => {
                log::debug!(
                    "PartitionKind::set_code set partition type code to: {}",
                    code
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set partition type code to: {}", code);
                log::debug!("PartitionKind::set_code {}. libfdisk::fdisk_parttype_set_code returned error code: {:?}", err_msg, code);

                Err(PartitionKindError::Setting(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Sets a partition type's GUID.
    pub(crate) fn set_guid(&mut self, guid: Guid) -> Result<(), PartitionKindError> {
        let guid_cstr = guid.to_c_string();
        log::debug!(
            "PartitionKind::set_guid setting partition type GUID to: {:?}",
            guid
        );

        let result =
            unsafe { libfdisk::fdisk_parttype_set_typestr(self.inner, guid_cstr.as_ptr()) };

        match result {
            0 => {
                log::debug!(
                    "PartitionKind::set_guid set partition type GUID to: {:?}",
                    guid
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set partition type GUID to: {:?}", guid);
                log::debug!("PartitionKind::set_guid {}. libfdisk::fdisk_parttype_set_typestr returned error code: {:?}", err_msg, code);

                Err(PartitionKindError::Setting(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Sets a partition type's name.
    pub(crate) fn set_name<T>(&mut self, name: T) -> Result<(), PartitionKindError>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        let name_cstr = ffi_utils::as_ref_str_to_c_string(name).map_err(|e| {
            let err_msg = format!("failed to convert value to `CString` {e}");
            PartitionKindError::CStringConversion(err_msg)
        })?;

        log::debug!(
            "PartitionKind::set_name setting partition type's name to: {:?}",
            name
        );

        let result = unsafe { libfdisk::fdisk_parttype_set_name(self.inner, name_cstr.as_ptr()) };

        match result {
            0 => {
                log::debug!(
                    "PartitionKind::set_name set partition type's name to: {:?}",
                    name
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set partition type's name to: {:?}", name);
                log::debug!("PartitionKind::set_name {}. libfdisk::fdisk_parttype_set_name returned error code: {:?}", err_msg, code);

                Err(PartitionKindError::Setting(err_msg))
            }
        }
    }

    /// Creates a [`PartitionKindBuilder`] to configure and construct a new `PartitionKind` instance.
    ///
    /// Call the `PartitionKindBuilder`'s [`build()`](crate::core::partition::PartitionKindBuilder::build) method to
    /// instantiate a new `PartitionKind`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsfdisk::core::partition::PartitionKind;
    /// use rsfdisk::core::partition::Code;
    /// use rsfdisk::core::partition::Guid;
    ///
    /// fn main() -> rsfdisk::Result<()> {
    ///     // MBR partition type
    ///     let name = "Linux Root";
    ///     let partition_kind = PartitionKind::builder()
    ///         .code(Code::Linux)
    ///         .name(name)
    ///         .build()?;
    ///
    ///     let actual = partition_kind.code();
    ///     let code = Code::Linux.to_u32();
    ///     let expected = Some(code);
    ///     assert_eq!(actual, expected);
    ///
    ///     let actual = partition_kind.name();
    ///     let expected = Some(name);
    ///     assert_eq!(actual, expected);
    ///
    ///     // GPT partition type
    ///     let name = "Solaris Root";
    ///     let partition_kind = PartitionKind::builder()
    ///         .guid(Guid::SolarisRoot)
    ///         .name(name)
    ///         .build()?;
    ///
    ///     let actual = partition_kind.guid();
    ///     let guid = Guid::SolarisRoot.as_str();
    ///     let expected = Some(guid);
    ///     assert_eq!(actual, expected);
    ///
    ///     let actual = partition_kind.name();
    ///     let expected = Some(name);
    ///     assert_eq!(actual, expected);
    ///
    ///     // A custom partition type
    ///     let code = 0x1234;
    ///     let type_string = "custom_root";
    ///     let name = "Custom Root";
    ///     let partition_kind = PartitionKind::builder()
    ///         .unknown_kind(code, type_string)
    ///         .name(name)
    ///         .build()?;
    ///
    ///     let actual = partition_kind.code();
    ///     let expected = Some(code);
    ///     assert_eq!(actual, expected);
    ///
    ///     let actual = partition_kind.guid();
    ///     let expected = Some(type_string);
    ///     assert_eq!(actual, expected);
    ///
    ///     let actual = partition_kind.name();
    ///     let expected = Some(name);
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn builder() -> PartitionKindBuilder {
        log::debug!("PartitionKind::builder creating a new `PartitionKindBuilder` instance");

        PartTypeBuilder::builder()
    }

    #[doc(hidden)]
    /// Copies a partition type.
    pub(crate) fn copy_partition_type(
        part_kind: &Self,
    ) -> Result<PartitionKind, PartitionKindError> {
        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_parttype>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_copy_parttype(part_kind.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to copy `PartitionKind`".to_owned();
                log::debug!("PartitionKind::copy_partition_type {}. libfdisk::fdisk_copy_parttype returned a NULL pointer", err_msg);

                Err(PartitionKindError::Copy(err_msg))
            }
            ptr => {
                log::debug!("PartitionKind::copy_partition_type copied partition type");
                let part_kind = Self::from_ptr(ptr);

                Ok(part_kind)
            }
        }
    }

    #[doc(hidden)]
    /// Returns a copy of this `PartitionKind`.
    fn copy(&self) -> Result<PartitionKind, PartitionKindError> {
        log::debug!("PartitionKind::copy copying partition type");

        Self::copy_partition_type(self)
    }

    /// Returns the partition type's identification code.
    pub fn code(&self) -> Option<u32> {
        let code = unsafe { libfdisk::fdisk_parttype_get_code(self.inner) };
        log::debug!(
            "PartitionKind::code partition identification code: 0x{:x}",
            code
        );

        match code {
            0 => None,
            _ => Some(code),
        }
    }

    /// Returns the partition type's string identifier (e.g. the GUID for a `GPT` partition type).
    pub fn guid(&self) -> Option<&str> {
        log::debug!("PartitionKind::king getting partition guid");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_parttype_get_string(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("PartitionKind::guid no partition guid. libfdisk::fdisk_parttype_get_string returned a NULL pointer");

                None
            }
            guid_ptr => {
                let guid = ffi_utils::const_char_array_to_str_ref(guid_ptr).ok();
                log::debug!("PartitionKind::guid value: {:?}", guid);

                guid
            }
        }
    }

    /// Returns the partition type's name.
    pub fn name(&self) -> Option<&str> {
        log::debug!("PartitionKind::name getting partition type's name");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_parttype_get_name(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("PartitionKind::name no name provided. libfdisk::fdisk_parttype_get_name returned a NULL pointer");

                None
            }
            name_ptr => {
                let name = ffi_utils::const_char_array_to_str_ref(name_ptr).ok();
                log::debug!("PartitionKind::name value: {:?}", name);

                name
            }
        }
    }

    /// Returns `true` when the partition type is classified as `unknown`.
    pub fn is_unknown_type(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_parttype_is_unknown(self.inner) == 1 };
        log::debug!("PartitionKind::is_unknown_type value: {:?}", state);

        state
    }
}

impl AsRef<PartitionKind> for PartitionKind {
    #[inline]
    fn as_ref(&self) -> &PartitionKind {
        self
    }
}

impl Clone for PartitionKind {
    /// Returns a copy of this `PartitionKind`.
    fn clone(&self) -> PartitionKind {
        self.copy().unwrap()
    }
}

impl Drop for PartitionKind {
    fn drop(&mut self) {
        log::debug!("PartitionKind::drop deallocating `PartitionKind` instance");

        unsafe { libfdisk::fdisk_unref_parttype(self.inner) }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(
        expected = "you must call one of the following methods: `code`, `guid`, or `unknown_kind`"
    )]
    fn partition_kind_one_of_code_guid_or_unknown_kind_must_be_set() {
        let _ = PartitionKind::builder().build().unwrap();
    }

    #[test]
    #[should_panic(
        expected = "methods `code`, `guid`, and `unknown_kind` can not be called at the same time"
    )]
    fn partition_kind_code_and_guid_are_mutually_exclusive() {
        let _ = PartitionKind::builder()
            .code(Code::Linux)
            .guid(Guid::SolarisRoot)
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic(
        expected = "methods `code`, `guid`, and `unknown_kind` can not be called at the same time"
    )]
    fn partition_kind_code_and_unknown_kind_are_mutually_exclusive() {
        let _ = PartitionKind::builder()
            .code(Code::Linux)
            .unknown_kind(0x1234, "unknown")
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic(
        expected = "methods `code`, `guid`, and `unknown_kind` can not be called at the same time"
    )]
    fn partition_kind_guid_and_unknown_kind_are_mutually_exclusive() {
        let _ = PartitionKind::builder()
            .code(Code::Linux)
            .guid(Guid::SolarisRoot)
            .build()
            .unwrap();
    }

    #[test]
    fn partition_kind_can_create_an_mbr_partition_kind() -> crate::Result<()> {
        let name = "Linux Root";
        let partition_kind = PartitionKind::builder()
            .code(Code::Linux)
            .name(name)
            .build()?;

        let actual = partition_kind.code();
        let code = Code::Linux.to_u32();
        let expected = Some(code);
        assert_eq!(actual, expected);

        let actual = partition_kind.name();
        let expected = Some(name);
        assert_eq!(actual, expected);

        let actual = partition_kind.guid();
        let expected = None;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_kind_can_create_a_gpt_partition_kind() -> crate::Result<()> {
        let name = "Solaris Root";
        let partition_kind = PartitionKind::builder()
            .guid(Guid::SolarisRoot)
            .name(name)
            .build()?;

        let actual = partition_kind.guid();
        let guid = Guid::SolarisRoot.as_str();
        let expected = Some(guid);
        assert_eq!(actual, expected);

        let actual = partition_kind.name();
        let expected = Some(name);
        assert_eq!(actual, expected);

        let actual = partition_kind.code();
        let expected = None;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_kind_can_create_a_custom_partition_kind() -> crate::Result<()> {
        let code = 0x1234;
        let type_string = "custom_root";
        let name = "Custom Root";
        let partition_kind = PartitionKind::builder()
            .unknown_kind(code, type_string)
            .name(name)
            .build()?;

        let actual = partition_kind.guid();
        let expected = Some(type_string);
        assert_eq!(actual, expected);

        let actual = partition_kind.name();
        let expected = Some(name);
        assert_eq!(actual, expected);

        let actual = partition_kind.code();
        let expected = Some(code);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_kind_can_clone_a_partition_kind() -> crate::Result<()> {
        let name = "Solaris Root";
        let partition_kind = PartitionKind::builder()
            .guid(Guid::SolarisRoot)
            .name(name)
            .build()?;

        let copy = partition_kind.clone();

        let actual = copy.guid();
        let expected = partition_kind.guid();
        assert_eq!(actual, expected);

        let actual = copy.name();
        let expected = partition_kind.name();
        assert_eq!(actual, expected);

        let actual = copy.code();
        let expected = partition_kind.code();
        assert_eq!(actual, expected);

        Ok(())
    }
}
