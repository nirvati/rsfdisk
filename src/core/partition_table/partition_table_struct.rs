// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::errors::PartitionTableError;

use crate::core::partition::PartitionKind;
use crate::core::partition_table::Field;
use crate::core::partition_table::FieldFormat;
use crate::core::partition_table::InputType;
use crate::core::partition_table::PartitionTableKind;
use crate::core::partition_table::Range;
use crate::core::partition_table::Shortcut;

use crate::ffi_to_string_or_empty;
use crate::ffi_utils;

/// A partition table.
#[derive(Debug)]
#[repr(transparent)]
pub struct PartitionTable {
    pub(crate) inner: *mut libfdisk::fdisk_label,
}

impl PartitionTable {
    #[doc(hidden)]
    #[allow(dead_code)]
    /// Wraps a raw `libfdisk::fdisk_label` with a safe `PartitionTable`.
    pub(crate) fn from_ptr(ptr: *mut libfdisk::fdisk_label) -> PartitionTable {
        Self { inner: ptr }
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Wraps a boxed raw `libfdisk::fdisk_label` pointer in a safe reference.
    pub(crate) unsafe fn ref_from_boxed_ptr<'a>(
        ptr: Box<*mut libfdisk::fdisk_label>,
    ) -> (*mut *mut libfdisk::fdisk_label, &'a Self) {
        let raw_ptr = Box::into_raw(ptr);
        let entry_ref = unsafe { &*(raw_ptr as *const _ as *const Self) };

        (raw_ptr, entry_ref)
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Wraps a boxed raw `libfdisk::fdisk_label` pointer in a safe reference.
    pub(crate) unsafe fn mut_from_boxed_ptr<'a>(
        ptr: Box<*mut libfdisk::fdisk_label>,
    ) -> (*mut *mut libfdisk::fdisk_label, &'a mut Self) {
        let raw_ptr = Box::into_raw(ptr);
        let entry_ref = unsafe { &mut *(raw_ptr as *mut Self) };

        (raw_ptr, entry_ref)
    }

    /// Returns this `PartitionTable`'s type.
    pub fn kind(&self) -> PartitionTableKind {
        let code = unsafe { libfdisk::fdisk_label_get_type(self.inner) };
        // It is safe to assume, `libfdisk` will not return an unsupported ID.
        let kind = PartitionTableKind::try_from(code as u32)
            .map_err(|e| {
                let err_msg = format!("unsupported partition table type. code: {}. {}", code, e);
                PartitionTableError::PartitionTableKind(err_msg)
            })
            .unwrap();

        log::debug!(
            "PartitionTable::kind values code: {:?}, kind: {:?}",
            code,
            kind
        );

        kind
    }

    /// Returns the [`PartitionTable`]'s name.
    pub fn name(&self) -> Option<&str> {
        log::debug!("PartitionTable::name getting partition table's name");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_label_get_name(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("PartitionTable::name no name provided. libfdisk::fdisk_label_get_name returned a NULL pointer");

                None
            }
            name_ptr => {
                let name = ffi_utils::const_char_array_to_str_ref(name_ptr).ok();

                log::debug!(
                    "PartitionTable::name got partition table's name: {:?}",
                    name,
                );

                name
            }
        }
    }

    /// Returns the [`Range`] of admissible cylinder values.
    pub fn geometry_cylinders(&self) -> Option<Range> {
        log::debug!(
            "PartitionTable::geometry_cylinders getting admissible range for cylinder values"
        );

        let mut min = MaybeUninit::<libfdisk::fdisk_sector_t>::zeroed();
        let mut max = MaybeUninit::<libfdisk::fdisk_sector_t>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_label_get_geomrange_cylinders(
                self.inner,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            )
        };

        match result {
            0 => {
                let lower_bound = unsafe { min.assume_init() };
                let upper_bound = unsafe { max.assume_init() };
                let range = Range::new(lower_bound, upper_bound);

                log::debug!("PartitionTable::geometry_cylinders got admissible range for cylinder values: {:?}", range);

                Some(range)
            }
            code => {
                let err_msg = "failed to get admissible range for cylinder values".to_owned();
                log::debug!("PartitionTable::geometry_cylinders {}. libfdisk::fdisk_label_get_geomrange_cylinders returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    /// Returns the [`Range`] of admissible head values.
    pub fn geometry_heads(&self) -> Option<Range> {
        log::debug!("PartitionTable::geometry_heads getting admissible range for head values");

        let mut min = MaybeUninit::<libc::c_uint>::zeroed();
        let mut max = MaybeUninit::<libc::c_uint>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_label_get_geomrange_heads(
                self.inner,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            )
        };

        match result {
            0 => {
                let lower_bound = unsafe { min.assume_init() as u64 };
                let upper_bound = unsafe { max.assume_init() as u64 };
                let range = Range::new(lower_bound, upper_bound);

                log::debug!(
                    "PartitionTable::geometry_heads got admissible range for head values: {:?}",
                    range
                );

                Some(range)
            }
            code => {
                let err_msg = "failed to get admissible range for head values".to_owned();
                log::debug!("PartitionTable::geometry_heads {}. libfdisk::fdisk_label_get_geomrange_heads returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    /// Returns the [`Range`] of admissible sector values.
    pub fn geometry_sectors(&self) -> Option<Range> {
        log::debug!("PartitionTable::geometry_sectors getting admissible range for sector values");

        let mut min = MaybeUninit::<libfdisk::fdisk_sector_t>::zeroed();
        let mut max = MaybeUninit::<libfdisk::fdisk_sector_t>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_label_get_geomrange_sectors(
                self.inner,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            )
        };

        match result {
            0 => {
                let lower_bound = unsafe { min.assume_init() };
                let upper_bound = unsafe { max.assume_init() };
                let range = Range::new(lower_bound, upper_bound);

                log::debug!(
                    "PartitionTable::geometry_sectors got admissible range for sector values: {:?}",
                    range
                );

                Some(range)
            }
            code => {
                let err_msg = "failed to get admissible range for sector values".to_owned();
                log::debug!("PartitionTable::geometry_sectors {}. libfdisk::fdisk_label_get_geomrange_sectors returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    /// Marks this `PartitionTable` as `disabled`, the library will ignore this partition table
    /// when scanning for partition tables on a device.
    pub fn disable(&self) {
        log::debug!("PartitionTable::disable disabling partition table");

        unsafe { libfdisk::fdisk_label_set_disabled(self.inner, 1) }
    }

    /// Marks this `PartitionTable` as `enabled`.
    pub fn enable(&mut self) {
        log::debug!("PartitionTable::enable disabling partition table");

        unsafe { libfdisk::fdisk_label_set_disabled(self.inner, 0) }
    }

    /// Returns the parameters for formatting a [`Field`] when printing it on the terminal.
    pub fn partition_field_format(&self, field: Field) -> Option<FieldFormat> {
        log::debug!(
            "PartitionTable::partition_field_format getting Partition Entry Array's formatting parameters: {:?}",
            field
        );

        let field_int = field as u32 as i32;

        let mut ptr = MaybeUninit::<*const libfdisk::fdisk_field>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_label_get_field(self.inner, field_int));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!(
                    "failed to get Partition Entry Array's formatting parameters: {:?}",
                    field
                );
                log::debug!("PartitionTable::partition_field_format {}. libfdisk::fdisk_label_get_field  returned a NULL pointer", err_msg);

                None
            }
            ptr => {
                log::debug!(
                    "PartitionTable::partition_field_format got Partition Entry Array's formatting parameters: {:?}",
                    field
                );
                let format = FieldFormat::from_ptr(ptr);

                Some(format)
            }
        }
    }

    /// Returns the parameters for formatting a partition entry field matching the given `name`
    /// when printing it on the terminal.
    pub fn partition_field_format_by_name<T>(&self, name: T) -> Option<FieldFormat>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        let name_cstr = ffi_utils::as_ref_str_to_c_string(name).ok()?;
        log::debug!(
            "PartitionTable::partition_field_format_by_name getting Partition Entry Array's formatting parameters: {:?}",
            name
        );

        let mut ptr = MaybeUninit::<*const libfdisk::fdisk_field>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_label_get_field_by_name(
                self.inner,
                name_cstr.as_ptr(),
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!(
                    "failed to get Partition Entry Array's formatting parameters: {:?}",
                    name
                );
                log::debug!("PartitionTable::partition_field_format_by_name {}. libfdisk::fdisk_label_get_field_by_name returned a NULL pointer", err_msg);

                None
            }
            ptr => {
                log::debug!(
                    "PartitionTable::partition_field_format_by_name got partition formatting parameters: {:?}",
                    name
                );
                let field = FieldFormat::from_ptr(ptr);

                Some(field)
            }
        }
    }

    /// Returns the number of partition types supported by this `PartitionTable`.
    pub fn count_supported_partition_types(&self) -> usize {
        let count = unsafe { libfdisk::fdisk_label_get_nparttypes(self.inner) };
        log::debug!(
            "PartitionTable::count_supported_partition_types value: {:?}",
            count
        );

        count
    }

    /// Returns the `nth` supported partition type.
    pub fn supported_partition_types(&self, nth: usize) -> Option<PartitionKind> {
        log::debug!("PartitionTable::nth_supported_partition_types getting supported partition type number: {:?}", nth);

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_parttype>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_label_get_parttype(self.inner, nth));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("PartitionTable::supported_partition_types no partition type at index: {:?}. libfdisk::fdisk_label_get_parttype returned a NULL pointer", nth);

                None
            }
            ptr => {
                log::debug!("PartitionTable::nth_supported_partition_types got supported partition type number: {:?}", nth);
                let kind = PartitionKind::from_ptr(ptr);

                Some(kind)
            }
        }
    }

    /// Returns the [`Shortcut`] of the `nth` supported partition type.
    pub fn partition_type_shortcut(&self, nth: usize) -> Option<Shortcut> {
        log::debug!("PartitionTable::partition_type_shortcut getting shortcut for supported partition type number: {:?}", nth);

        let mut typestr = MaybeUninit::<*const libc::c_char>::zeroed();
        let mut shortcut = MaybeUninit::<*const libc::c_char>::zeroed();
        let mut alias = MaybeUninit::<*const libc::c_char>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_label_get_parttype_shortcut(
                self.inner,
                nth,
                typestr.as_mut_ptr(),
                shortcut.as_mut_ptr(),
                alias.as_mut_ptr(),
            )
        };

        match result {
            code if code == 0 || code == 2 => {
                let typestr_cstr = unsafe { typestr.assume_init() };
                let shortcut_cstr = unsafe { shortcut.assume_init() };
                let alias_cstr = unsafe { alias.assume_init() };

                let type_string = ffi_to_string_or_empty!(typestr_cstr);
                let shortcut = ffi_to_string_or_empty!(shortcut_cstr);
                let alias = ffi_to_string_or_empty!(alias_cstr);
                let deprecated_alias = code == 2;

                let shortcut = Shortcut::new(alias, shortcut, type_string, deprecated_alias);

                Some(shortcut)
            }
            code => {
                let reason = if code == 1 {
                    format!(", index {} out of bounds", nth)
                } else {
                    String::new()
                };
                let err_msg = format!(
                    "failed to get shortcut for partition type number: {:?}{}",
                    nth, reason
                );
                log::debug!("PartitionTable::partition_type_shortcut {}. libfdisk::fdisk_label_get_parttype_shortcut returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    /// Each partition table type supports specific kinds of partitions. These partition types have
    /// various identification schemes, for example the type of a partition in:
    /// - an `MBR` partition table is referred to by a numerical code in hexadecimal (i.e. `0x83`
    /// for a Linux native partition, `0x39` for a Plan 9 edition 3 partition, etc.),
    /// - a `GPT` partition table is characterized by a partition UUID (i.e.
    /// `83bd6b9d-7f41-11dc-be0b-001560b84f0f` for a FreeBSD boot partition)
    ///
    /// Given a string identifier, this function will try to parse it into the corresponding
    /// [`PartitionKind`] supported by this `PartitionTable`. If the identifier is unknown, this
    /// method will return an "unknown" `PartitionKind` for which [`PartitionKind::is_unknown_type`]
    /// will return `true`.
    ///
    /// **Note:** for `GPT` partition tables, this function accepts the same sequence value (as a
    /// string) as the one used by [`PartitionTable::supported_partition_types`] to identify a
    /// supported [`PartitionKind`].
    pub fn partition_type_from_string_id<T>(
        &self,
        id: T,
    ) -> Result<PartitionKind, PartitionTableError>
    where
        T: AsRef<str>,
    {
        let id = id.as_ref();
        let id_cstr = ffi_utils::as_ref_str_to_c_string(id)?;
        log::debug!(
            "PartitionTable::partition_type_from_string_id parsing: {:?} into a `PartitionKind`",
            id
        );

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_parttype>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_label_parse_parttype(
                self.inner,
                id_cstr.as_ptr(),
            ));
        }
        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!("failed to parse: {:?} into a partition type", id);
                log::debug!("PartitionTable::partition_type_from_string_id {}. libfdisk::fdisk_label_get_parttype_from_string returned a NULL pointer", err_msg);

                Err(PartitionTableError::Parse(err_msg))
            }
            ptr => {
                log::debug!(
                    "PartitionTable::partition_type_from_string_id parsed id: {:?} into a partition type",
                    id
                );
                let kind = PartitionKind::from_ptr(ptr);

                Ok(kind)
            }
        }
    }

    /// Each partition table type supports specific kinds of partitions which follow various
    /// identification schemes.  Given a string identifier, this generic parser will try to turn it
    /// into the right [`PartitionKind`] supported by this `PartitionTable`. You can provide a list
    /// of [`InputType`] hints to direct the parser, and narrow the search.
    ///
    /// This identifier string can be any of:
    /// - a hexadecimal code,
    /// - a UUID,
    /// - an alias (i.e. `linux` for a Linux partition),
    /// - a shortcut (i.e. `L` for a Linux partition).
    ///
    /// This method will return, unless told otherwise, an "unknown" `PartitionKind` for
    /// which [`PartitionKind::is_unknown_type`] will return `true` if it can't identify the
    /// corresponding partition type.
    ///
    /// **Note:** for `GPT` partition tables, this function accepts the same sequence value (as a
    /// string) as the one used by [`PartitionTable::supported_partition_types`] to identify a
    /// supported [`PartitionKind`].
    pub fn partition_type_parse<T, U>(
        &self,
        string: T,
        flags: U,
    ) -> Result<PartitionKind, PartitionTableError>
    where
        T: AsRef<str>,
        U: AsRef<[InputType]>,
    {
        let string = string.as_ref();
        let flags = flags.as_ref();

        let string_cstr = ffi_utils::as_ref_str_to_c_string(string)?;
        let c_flags = flags.iter().fold(0, |acc, &flag| acc | flag as u32);

        log::debug!("PartitionTable::partition_type_parse parsing: {:?} with flags: {:?} into a `PartitionKind`", string, flags);

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_parttype>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_label_advparse_parttype(
                self.inner,
                string_cstr.as_ptr(),
                c_flags as i32,
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!(
                    "failed to parse {:?} with flags {:?} into a partition type",
                    string, flags
                );
                log::debug!("PartitionTable::partition_type_parse {}. libfdisk::fdisk_label_advparse_parttype returned a NULL pointer", err_msg);

                Err(PartitionTableError::Parse(err_msg))
            }
            ptr => {
                log::debug!("PartitionTable::partition_type_parse parsed: {:?} with flags: {:?} into a `PartitionKind`", string, flags);
                let kind = PartitionKind::from_ptr(ptr);

                Ok(kind)
            }
        }
    }

    /// Converts a partition type's identification `code` into a [`PartitionKind`].
    pub fn partition_type_from_code(
        &self,
        code: u32,
    ) -> Result<PartitionKind, PartitionTableError> {
        log::debug!(
            "PartitionTable::partition_type_from_code converting code: {:?} to partition type",
            code
        );

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_parttype>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_label_get_parttype_from_code(
                self.inner, code,
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!("failed to convert code: {:?} to partition type", code);
                log::debug!("PartitionTable::partition_type_from_code {}. libfdisk::fdisk_label_get_parttype_from_code returned a NULL pointer", err_msg);

                Err(PartitionTableError::Conversion(err_msg))
            }
            ptr => {
                log::debug!("PartitionTable::partition_type_from_code converted code: {:?} to partition type", code);
                let kind = PartitionKind::from_ptr(ptr);

                Ok(kind)
            }
        }
    }

    /// Converts a `string` partition type identifier into a [`PartitionKind`].
    pub fn partition_type_from_string<T>(
        &self,
        string: T,
    ) -> Result<PartitionKind, PartitionTableError>
    where
        T: AsRef<str>,
    {
        let string = string.as_ref();
        let string_cstr = ffi_utils::as_ref_str_to_c_string(string)?;
        log::debug!(
            "PartitionTable::partition_type_from_code converting string: {:?} to partition type",
            string
        );

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_parttype>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_label_get_parttype_from_string(
                self.inner,
                string_cstr.as_ptr(),
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!("failed to convert string: {:?} to partition type", string);
                log::debug!("PartitionTable::partition_type_from_code {}. libfdisk::fdisk_label_get_parttype_from_string returned a NULL pointer", err_msg);

                Err(PartitionTableError::Parse(err_msg))
            }
            ptr => {
                log::debug!("PartitionTable::partition_type_from_code converted string: {:?} to partition type", string);
                let kind = PartitionKind::from_ptr(ptr);

                Ok(kind)
            }
        }
    }

    /// `PartitionTable` keeps track of changes, so calling this function is not required unless
    /// you want to force an `Fdisk` instance to use the current state of this `PartitionTable`
    /// when writing data to disk.
    pub fn mark_as_changed(&mut self) {
        log::debug!("PartitionTable::mark_as_changed marking partition table as changed");

        unsafe { libfdisk::fdisk_label_set_changed(self.inner, 1) }
    }

    /// Marks the in-memory partition table as `unchanged`.
    pub fn mark_as_unchanged(&mut self) {
        log::debug!("PartitionTable::mark_as_unchanged marking partition table as unchanged");

        unsafe { libfdisk::fdisk_label_set_changed(self.inner, 0) }
    }

    //---- END mutators

    //---- BEGIN predicates

    /// Returns `true` when this `PartitionTable` is marked as `disabled`.
    pub fn is_disabled(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_label_is_disabled(self.inner) == 1 };
        log::debug!("PartitionTable::is_disabled value: {:?}", state);

        state
    }

    /// Returns `true` when the in-memory `PartitionTable` has been modified.
    pub fn has_changes(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_label_is_changed(self.inner) == 1 };
        log::debug!("PartitionTable::has_changes value: {:?}", state);

        state
    }

    /// Returns `true` when this `PartitionTable` requires accessing partitions by Cylinder-Head-Sector (CHS) addressing.
    pub fn requires_chs_addressing(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_label_require_geometry(self.inner) == 1 };
        log::debug!("PartitionTable::requires_chs_addressing value: {:?}", state);

        state
    }

    /// Returns `true` when this `PartitionTable` supports [`Shortcut`]s/aliases for partition
    /// types.
    pub fn supports_partition_type_shortcuts(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_label_has_parttypes_shortcuts(self.inner) == 1 };
        log::debug!(
            "PartitionTable::has_partition_type_shortcuts value: {:?}",
            state
        );

        state
    }

    /// Returns `true` when this `PartitionTable` uses codes to identify partition types (e.g.
    /// the [`OSType`
    /// field](https://uefi.org/specs/UEFI/2.10/05_GUID_Partition_Table_Format.html#legacy-mbr-partition-record)
    /// in an `MBR` Partition Entry).
    pub fn uses_partition_type_codes(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_label_has_code_parttypes(self.inner) == 1 };
        log::debug!(
            "PartitionTable::uses_partition_type_codes value: {:?}",
            state
        );

        state
    }

    //---- END predicates
}

impl AsRef<PartitionTable> for PartitionTable {
    #[inline]
    fn as_ref(&self) -> &PartitionTable {
        self
    }
}
