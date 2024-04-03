// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::partition_table::Field;
use crate::core::partition_table::MaxColWidth;
use crate::ffi_utils;

/// The formatting parameters for printing a [`Field`].
#[derive(Debug)]
pub struct FieldFormat {
    inner: *const libfdisk::fdisk_field,
}

impl FieldFormat {
    #[doc(hidden)]
    #[allow(dead_code)]
    /// Wraps a raw `libfdisk::fdisk_field` with a safe `FieldFormat`.
    pub(super) fn from_ptr(ptr: *const libfdisk::fdisk_field) -> FieldFormat {
        Self { inner: ptr }
    }

    /// Returns the [`Field`] to which the formatting parameters must be applied.
    pub fn field(&self) -> Field {
        let code = unsafe { libfdisk::fdisk_field_get_id(self.inner) as u32 };

        log::debug!("FieldFormat::id code: 0x{:x}", code);

        Field::try_from(code).ok().unwrap()
    }

    /// Returns the name of the column to which the [`Field`] belongs.
    pub fn col_name(&self) -> Option<&str> {
        log::debug!("FieldFormat::name getting partition name");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_field_get_name(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("FieldFormat::name provided no name. libfdisk::fdisk_field_get_name returned a NULL pointer");

                None
            }
            name_ptr => {
                let name = ffi_utils::const_char_array_to_str_ref(name_ptr).ok();
                log::debug!("FieldFormat::name value: {:?}", name);

                name
            }
        }
    }

    /// Returns the maximum column width when printing the field's content to the terminal.
    pub fn width(&self) -> Option<MaxColWidth> {
        let width = unsafe { libfdisk::fdisk_field_get_width(self.inner) };
        log::debug!("FieldFormat::width value: {:?}", width);

        MaxColWidth::try_from(width).ok()
    }

    /// Returns `true` if the [`Field`] contains a numerical value.
    pub fn is_numeric(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_field_is_number(self.inner) == 1 };
        log::debug!("FieldFormat::is_numeric value: {:?}", state);

        state
    }
}
