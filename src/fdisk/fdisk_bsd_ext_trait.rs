// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::private::Sealed;
use crate::fdisk::Fdisk;
use crate::fdisk::FdiskError;

/// `BSD` specific functions for a [`Fdisk`].
///
/// This trait is sealed and can not be implemented for types outside of `rsfdisk`.
pub trait FdiskBSDExt: Sealed {
    /// Edits the fields of a `BSD disklabel`.
    fn bsd_edit_disk_label(&mut self) -> Result<(), FdiskError>;

    /// Sets a `DOS` partition to a nested `BSD` partition table as its parent.
    fn bsd_link_to_nested_partition(&mut self) -> Result<(), FdiskError>;

    /// Installs a `BSD` bootstrap file on the assigned device.
    fn bsd_install_bootstrap_file(&mut self) -> Result<(), FdiskError>;
}

impl<'a> FdiskBSDExt for Fdisk<'a> {
    fn bsd_edit_disk_label(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::bsd_edit_disk_label editing BSD disklabel");

        let result = unsafe { libfdisk::fdisk_bsd_edit_disklabel(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::bsd_edit_disk_label edited BSD disklabel");

                Ok(())
            }
            code => {
                let err_msg = "failed to edit BSD disklabel".to_owned();
                log::debug!("Fdisk::bsd_edit_disk_label {}. libfdisk::fdisk_bsd_edit_disklabel returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    fn bsd_link_to_nested_partition(&mut self) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::bsd_link_to_nested_partition linking DOS parent to BSD nested partition table"
        );

        let result = unsafe { libfdisk::fdisk_bsd_link_partition(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::bsd_link_to_nested_partition linked DOS parent to BSD nested partition table");

                Ok(())
            }
            code => {
                let err_msg = "failed to link DOS parent to BSD nested partition table".to_owned();
                log::debug!("Fdisk::bsd_link_to_nested_partition {}. libfdisk::fdisk_bsd_link_partition returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    fn bsd_install_bootstrap_file(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::bsd_install_bootstrap_file installing BSD bootstrap file on device");

        let result = unsafe { libfdisk::fdisk_bsd_write_bootstrap(self.inner) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::bsd_install_bootstrap_file installed BSD bootstrap file on device"
                );

                Ok(())
            }
            code => {
                let err_msg = "failed to install BSD bootstrap file on device".to_owned();
                log::debug!("Fdisk::bsd_install_bootstrap_file {}. libfdisk::fdisk_bsd_write_bootstrap returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }
}
