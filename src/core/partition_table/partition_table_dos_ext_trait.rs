// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::errors::PartitionTableError;
use crate::core::partition_table::PartitionTable;
use crate::core::private::Sealed;

/// `DOS` specific functions for [`PartitionTable`]s
///
/// This trait is sealed and can not be implemented for types outside of `rsfdisk`.
pub trait PartitionTableDOSExt: Sealed {
    /// Enables deprecated `DOS` compatible mode. In this mode the library checks cylinder
    /// boundaries, CHS addressing characteristics, etc.
    fn dos_enable_compatible_mode(&mut self) -> Result<(), PartitionTableError>;

    /// Disables deprecated `DOS` compatible mode.
    fn dos_disable_compatible_mode(&mut self) -> Result<(), PartitionTableError>;

    /// Returns `true` if `DOS` compatible mode is enabled.
    fn is_dos_compatible(&self) -> bool;
}

fn set_dos_compatible_mode(
    ptr: *mut libfdisk::fdisk_label,
    enable: bool,
) -> Result<(), PartitionTableError> {
    let op = if enable { 1 } else { 0 };
    let op_str = if enable {
        "enable".to_owned()
    } else {
        "disable".to_owned()
    };

    let result = unsafe { libfdisk::fdisk_dos_enable_compatible(ptr, op) };

    match result {
        0 => {
            log::debug!(
                "PartitionTableDOSExt::set_dos_compatible_mode {}d DOS compatible mode",
                op_str
            );

            Ok(())
        }
        code => {
            let err_msg = format!("failed to {} DOS compatible mode", op_str);
            log::debug!("PartitionTableDOSExt::set_dos_compatible_mode {}. libfdisk::fdisk_dos_enable_compatible returned error code: {:?}", err_msg, code);

            Err(PartitionTableError::Config(err_msg))
        }
    }
}

impl PartitionTableDOSExt for PartitionTable {
    fn dos_enable_compatible_mode(&mut self) -> Result<(), PartitionTableError> {
        log::debug!("PartitionTable::dos_enable_compatible_mode enabling DOS compatible mode");

        set_dos_compatible_mode(self.inner, true)
    }

    fn dos_disable_compatible_mode(&mut self) -> Result<(), PartitionTableError> {
        log::debug!("PartitionTable::dos_disable_compatible_mode disabling DOS compatible mode");

        set_dos_compatible_mode(self.inner, false)
    }

    fn is_dos_compatible(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_dos_is_compatible(self.inner) == 1 };
        log::debug!("PartitionTable::is_dos_compatible value: {:?}", state);

        state
    }
}
