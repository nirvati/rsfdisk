// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::private::Sealed;
use crate::fdisk::Fdisk;
use crate::fdisk::FdiskError;

/// `SUN` specific functions for a [`Fdisk`].
///
/// This trait is sealed and can not be implemented for types outside of `rsfdisk`.
pub trait FdiskSUNExt: Sealed {
    /// Sets the assigned device's number of alternate cylinders by
    /// [`Prompt`](crate::core::prompt::Prompt)ing the user for a value.
    fn sun_set_alternate_cylinder_count(&mut self) -> Result<(), FdiskError>;

    /// Sets the assigned device's interleave factor by [`Prompt`](crate::core::prompt::Prompt)ing
    /// the user for a value.
    fn sun_set_interleave_factor(&mut self) -> Result<(), FdiskError>;

    /// Sets the assigned device's number of physical cylinders by
    /// [`Prompt`](crate::core::prompt::Prompt)ing the user for a value.
    fn sun_set_physical_cylinder_count(&mut self) -> Result<(), FdiskError>;

    /// Sets the assigned device's rotation speed by [`Prompt`](crate::core::prompt::Prompt)ing the
    /// user for a value.
    fn sun_set_rotation_per_minute(&mut self) -> Result<(), FdiskError>;

    /// Sets the assigned device's number of extra sectors per cylinder by
    /// [`Prompt`](crate::core::prompt::Prompt)ing the user for a value.
    fn sun_set_extra_sectors_per_cylinder(&mut self) -> Result<(), FdiskError>;
}

impl<'a> FdiskSUNExt for Fdisk<'a> {
    fn sun_set_alternate_cylinder_count(&mut self) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::sun_set_alternate_cylinder_count setting SUN number of alternate cylinders"
        );

        let result = unsafe { libfdisk::fdisk_sun_set_alt_cyl(self.inner) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::sun_set_alternate_cylinder_count set SUN number of alternate cylinders"
                );

                Ok(())
            }
            code => {
                let err_msg = "failed to set SUN number of alternate cylinders".to_owned();
                log::debug!("Fdisk::sun_set_alternate_cylinder_count {}. libfdisk::fdisk_sun_set_alt_cyl  returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    fn sun_set_interleave_factor(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::sun_set_interleave_factor setting SUN device interleave factor");

        let result = unsafe { libfdisk::fdisk_sun_set_ilfact(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::sun_set_interleave_factor set SUN device interleave factor");

                Ok(())
            }
            code => {
                let err_msg = "failed to set SUN device interleave factor".to_owned();
                log::debug!("Fdisk::sun_set_interleave_factor {}. libfdisk::fdisk_sun_set_ilfact returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    fn sun_set_physical_cylinder_count(&mut self) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::sun_set_physical_cylinder_count setting SUN number of physical cylinders"
        );

        let result = unsafe { libfdisk::fdisk_sun_set_pcylcount(self.inner) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::sun_set_physical_cylinder_count set SUN number of physical cylinders"
                );

                Ok(())
            }
            code => {
                let err_msg = "failed to set SUN number of physical cylinders".to_owned();
                log::debug!("Fdisk::sun_set_physical_cylinder_count {}. libfdisk::fdisk_sun_set_pcylcount returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    fn sun_set_rotation_per_minute(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::sun_set_rotation_per_minute setting SUN device rotation speed");

        let result = unsafe { libfdisk::fdisk_sun_set_rspeed(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::sun_set_rotation_per_minute set SUN device rotation speed");

                Ok(())
            }
            code => {
                let err_msg = "failed to set SUN device rotation speed".to_owned();
                log::debug!("Fdisk::sun_set_rotation_per_minute {}. libfdisk::fdisk_sun_set_rspeed returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    fn sun_set_extra_sectors_per_cylinder(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::sun_set_extra_sectors_per_cylinder setting SUN number of extra sectors per cylinder");

        let result = unsafe { libfdisk::fdisk_sun_set_xcyl(self.inner) };

        match result {
            0 => {
                log::debug!(
                        "Fdisk::sun_set_extra_sectors_per_cylinder set SUN number of extra sectors per cylinder"
                    );

                Ok(())
            }
            code => {
                let err_msg = "failed to set SUN number of extra sectors per cylinder".to_owned();
                log::debug!("Fdisk::sun_set_extra_sectors_per_cylinder {}. libfdisk::fdisk_sun_set_xcyl returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }
}
