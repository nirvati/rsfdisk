// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Miscellaneous `fdisk` utility functions.

// From dependency library

// From standard library
use std::mem::MaybeUninit;
use std::path::Path;

// From this library
use crate::ffi_utils;

/// Returns the name of a partition on a device from its identification number.
pub fn partition_name<T>(device_name: T, partition_number: usize) -> Option<String>
where
    T: AsRef<Path>,
{
    let device_name = device_name.as_ref();
    let device_name_cstr = ffi_utils::as_ref_path_to_c_string(device_name).ok()?;
    log::debug!(
        "misc::partition_name getting name of partition {:?} from device {:?}",
        partition_number,
        device_name
    );

    let mut ptr = MaybeUninit::<*mut libc::c_char>::zeroed();

    unsafe {
        ptr.write(libfdisk::fdisk_partname(
            device_name_cstr.as_ptr(),
            partition_number,
        ));
    }

    match unsafe { ptr.assume_init() } {
        ptr if ptr.is_null() => {
            log::debug!("misc::partition_name found no name for partition {:?} on device {:?}. libfdisk::fdisk_partname returned a NULL pointer", partition_number, device_name);

            None
        }
        name_ptr => {
            let partition_name = ffi_utils::c_char_array_to_string(name_ptr);
            log::debug!(
                "misc::partition_name partition {:?} on device {:?} is named {:?}",
                partition_number,
                device_name,
                partition_name
            );
            // Free memory allocated by `libfdisk`.
            unsafe {
                libc::free(name_ptr as *mut _);
            }

            Some(partition_name)
        }
    }
}
