// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::private::Sealed;
use crate::fdisk::Fdisk;
use crate::fdisk::FdiskError;

/// `SGI` specific functions for a [`Fdisk`].
///
/// This trait is sealed and can not be implemented for types outside of `rsfdisk`.
pub trait FdiskSGIExt: Sealed {
    /// Adds a hint to the first `SGI` volume (e.g. set `"sgilabel"` as the volume name) by
    /// [`Prompt`](crate::core::prompt::Prompt)ing the user for a value.
    fn sgi_add_hint(&mut self) -> Result<(), FdiskError>;

    /// Sets the SGI boot file by [`Prompt`](crate::core::prompt::Prompt)ing the user for a value.
    fn sgi_set_boot_file(&mut self) -> Result<(), FdiskError>;
}

impl<'a> FdiskSGIExt for Fdisk<'a> {
    fn sgi_add_hint(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::sgi_add_hint adding hint to SGI label");

        let result = unsafe { libfdisk::fdisk_sgi_create_info(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::sgi_add_hint added hint to SGI label");

                Ok(())
            }
            code => {
                let err_msg = "failed to add hint to SGI label".to_owned();
                log::debug!("Fdisk::sgi_add_hint {}. libfdisk::fdisk_sgi_create_info returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    fn sgi_set_boot_file(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::sgi_set_boot_file setting SGI boot file");

        let result = unsafe { libfdisk::fdisk_sgi_set_bootfile(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::sgi_set_boot_file set SGI boot file");

                Ok(())
            }
            code => {
                let err_msg = "failed to set SGI boot file".to_owned();
                log::debug!("Fdisk::sgi_set_boot_file {}. libfdisk::fdisk_sgi_set_bootfile returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }
}
