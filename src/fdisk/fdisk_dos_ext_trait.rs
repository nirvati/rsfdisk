// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::private::Sealed;
use crate::fdisk::Fdisk;
use crate::fdisk::FdiskError;

/// `DOS` specific functions for a [`Fdisk`].
///
/// This trait is sealed and can not be implemented for types outside of `rsfdisk`.
pub trait FdiskDOSExt: Sealed {
    #[cfg(fdisk = "v2_39")]
    /// Fixes the starting and ending LBA values for every partition according to their relative
    /// offset, size, and disk geometry (sectors per track and number of heads), then returns the
    /// number of modified partitions.
    fn dos_fix_chs_values(&mut self) -> usize;

    /// Interactively relocates a `DOS` partition on disk.
    fn dos_relocate_partition(&mut self, partition_number: usize) -> Result<(), FdiskError>;
}

impl<'a> FdiskDOSExt for Fdisk<'a> {
    #[cfg(fdisk = "v2_39")]
    fn dos_fix_chs_values(&mut self) -> usize {
        let modifications = unsafe { libfdisk::fdisk_dos_fix_chs(self.inner) as usize };
        log::debug!("Fdisk::dos_fix_chs_values fixing DOS Cylinder/Head/Sector values, modified {:?} values", modifications);

        modifications
    }

    fn dos_relocate_partition(&mut self, partition_number: usize) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::dos_relocate_partition relocating DOS partition {:?}",
            partition_number
        );

        let result = unsafe { libfdisk::fdisk_dos_move_begin(self.inner, partition_number) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::dos_relocate_partition relocated DOS partition {:?}",
                    partition_number
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to relocate DOS partition {:?}", partition_number);
                log::debug!("Fdisk::dos_relocate_partition {}. libfdisk::fdisk_dos_move_begin returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }
}
