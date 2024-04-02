// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::ffi::CString;
use std::fs::File;
use std::mem::MaybeUninit;
use std::os::fd::{BorrowedFd, IntoRawFd};
use std::path::Path;

// From this library
use crate::fdisk::CtxBuilder;
use crate::fdisk::DeviceAddressing;
use crate::fdisk::FdiskBuilder;
use crate::fdisk::FdiskError;
use crate::fdisk::GcItem;
use crate::fdisk::LBAAlign;
use crate::fdisk::SizeFormat;

use crate::core::partition::Partition;
use crate::core::partition::PartitionList;

use crate::core::partition_table::Field;

use crate::ffi_to_string_or_empty;
use crate::ffi_utils;
use crate::owning_mut_from_ptr;
use crate::owning_ref_from_ptr;

/// Partition table reader/editor/creator.
#[derive(Debug)]
pub struct Fdisk<'a> {
    pub(crate) inner: *mut libfdisk::fdisk_context,
    _parent: Option<&'a Fdisk<'a>>,
    pub(crate) gc: Vec<GcItem>,
}

impl<'a> Fdisk<'a> {
    #[doc(hidden)]
    /// Wraps a raw `libfdisk::fdisk_context` with a safe `Fdisk`.
    #[allow(dead_code)]
    pub(crate) fn from_ptr(
        ptr: *mut libfdisk::fdisk_context,
        parent: Option<&'a Fdisk<'a>>,
    ) -> Fdisk<'a> {
        Self {
            inner: ptr,
            _parent: parent,
            gc: vec![],
        }
    }

    #[doc(hidden)]
    /// Creates a default `Fdisk` instance.
    pub(crate) fn new() -> Result<Fdisk<'a>, FdiskError> {
        log::debug!("Fdisk::new creating a new `Fdisk` instance");

        let mut context = MaybeUninit::<*mut libfdisk::fdisk_context>::zeroed();
        unsafe {
            context.write(libfdisk::fdisk_new_context());
        }

        match unsafe { context.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `Fdisk` instance".to_owned();
                log::debug!(
                    "Fdisk::new {}. libfdisk::fdisk_new_context returned a NULL pointer",
                    err_msg
                );

                Err(FdiskError::Creation(err_msg))
            }
            ptr => {
                log::debug!("Fdisk::new created a new `Fdisk` instance");

                let context = Self::from_ptr(ptr, None);

                Ok(context)
            }
        }
    }

    #[doc(hidden)]
    /// Creates a new nested partitioner.
    fn make_new_nested_partitioner(
        parent: &'a Fdisk<'a>,
        name: &str,
    ) -> Result<Fdisk<'a>, FdiskError> {
        log::debug!("Fdisk::make_new_nested_partitioner creating a new nested `Fdisk` instance");
        let name_cstr = ffi_utils::as_ref_str_to_c_string(name)?;
        let name_ptr = if name.is_empty() {
            std::ptr::null()
        } else {
            name_cstr.as_ptr()
        };

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_context>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_new_nested_context(parent.inner, name_ptr));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new nested `Fdisk` instance".to_owned();
                log::debug!("Fdisk::make_new_nested_partitioner {}. libfdisk::fdisk_new_nested_context returned a NULL pointer", err_msg);

                Err(FdiskError::Creation(err_msg))
            }
            ptr => {
                log::debug!(
                    "Fdisk::make_new_nested_partitioner created a new nested `Fdisk` instance"
                );

                let nested_partitioner = Self::from_ptr(ptr, Some(parent));

                Ok(nested_partitioner)
            }
        }
    }

    /// Returns a new nested `Fdisk` for partition tables that support having nested partition tables (e.g. `BSD
    /// disklabels`, or `Protective MBR`).
    ///
    /// The returned `Fdisk` is initialized with data taken from its parent, sharing settings and
    /// the assigned device; changes to the nested context are propagated to its parent, but not the
    /// other way around.
    pub fn create_nested_partitioner(&'a mut self) -> Result<Fdisk, FdiskError> {
        Self::make_new_nested_partitioner(self, "")
    }

    /// Returns a new nested `Fdisk`, acts the same as [`Fdisk::create_nested_partitioner`]
    /// with the parameter `name` added as an attribute to the partition table (e.g. `"bsd"`).
    pub fn create_nested_partitioner_with_name<T>(
        &'a mut self,
        name: T,
    ) -> Result<Fdisk, FdiskError>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        Self::make_new_nested_partitioner(self, name)
    }

    //---- BEGIN setters

    #[doc(hidden)]
    /// Assigns a device to a `Fdisk` instance.
    fn assign_device<T>(fdisk: &mut Self, device_path: T, read_only: i32) -> Result<(), FdiskError>
    where
        T: AsRef<Path>,
    {
        let device_path = device_path.as_ref();
        let mode = if read_only == 0 {
            "read-write".to_owned()
        } else {
            "read-only".to_owned()
        };

        log::debug!(
            "Fdisk::assign_device assigning {} device: {:?}",
            mode,
            device_path
        );

        let dev_path = ffi_utils::as_ref_path_to_c_string(device_path)?;

        let result =
            unsafe { libfdisk::fdisk_assign_device(fdisk.inner, dev_path.as_ptr(), read_only) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::assign_device assigned {} device: {:?}",
                    mode,
                    device_path
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to assign {} device: {:?}", mode, device_path);
                log::debug!("Fdisk::assign_device {}. libfdisk::fdisk_assign_device returned error code: {:?}", err_msg, code);

                Err(FdiskError::AssignDevice(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Assigns a device by file to a `Fdisk` instance.
    fn assign_device_by_file<T>(
        fdisk: &mut Self,
        file: File,
        device_path: T,
        read_only: i32,
    ) -> Result<(), FdiskError>
    where
        T: AsRef<Path>,
    {
        let device_path = device_path.as_ref();
        let mode = if read_only == 0 {
            "read-write".to_owned()
        } else {
            "read-only".to_owned()
        };

        log::debug!(
            "Fdisk::assign_device_by_file assigning {} device: {:?}",
            mode,
            device_path
        );

        let dev_path = ffi_utils::as_ref_path_to_c_string(device_path)?;

        // Requested a read/write assignment but file open as read-only.
        if read_only == 0 && ffi_utils::is_open_read_only(&file)? {
            let err_msg = format!(
                "failed to assign {} device: {:?}. Device NOT open in read/write mode",
                mode, device_path
            );
            log::debug!("Fdisk::assign_device_by_file {}.", err_msg);

            Err(FdiskError::AssignDevice(err_msg))
        } else {
            let result = unsafe {
                libfdisk::fdisk_assign_device_by_fd(
                    fdisk.inner,
                    file.into_raw_fd(),
                    dev_path.as_ptr(),
                    read_only,
                )
            };

            match result {
                0 => {
                    log::debug!(
                        "Fdisk::assign_device_by_file assigned {} device: {:?}",
                        mode,
                        device_path
                    );

                    Ok(())
                }
                code => {
                    let err_msg = format!("failed to assign {} device: {:?}", mode, device_path);
                    log::debug!("Fdisk::assign_device_by_file {}. libfdisk::fdisk_assign_device_by_fd returned error code: {:?}", err_msg, code);

                    Err(FdiskError::AssignDevice(err_msg))
                }
            }
        }
    }

    #[doc(hidden)]
    /// Assigns a device in **read-only** mode to a `Fdisk`.
    pub(crate) fn assign_device_read_only<T>(&mut self, device_path: T) -> Result<(), FdiskError>
    where
        T: AsRef<Path>,
    {
        log::debug!(
            "Fdisk::assign_device_read_only assigning read-only device: {:?}",
            device_path.as_ref()
        );

        Self::assign_device(self, device_path, 1)
    }

    #[doc(hidden)]
    /// Assigns a device in **read-write** mode to a `Fdisk`.
    pub(crate) fn assign_device_read_write<T>(&mut self, device_path: T) -> Result<(), FdiskError>
    where
        T: AsRef<Path>,
    {
        log::debug!(
            "Fdisk::assign_device_read_write assigning read-write device: {:?}",
            device_path.as_ref()
        );

        Self::assign_device(self, device_path, 0)
    }

    #[doc(hidden)]
    /// Assigns a device by file in **read-only** mode to a `Fdisk`.
    pub(crate) fn assign_device_by_file_read_only<T>(
        &mut self,
        device_file: File,
        device_path: T,
    ) -> Result<(), FdiskError>
    where
        T: AsRef<Path>,
    {
        log::debug!(
            "Fdisk::assign_device_by_file_read_only assigning read-only device: {:?}",
            device_path.as_ref()
        );

        Self::assign_device_by_file(self, device_file, device_path, 1)
    }

    #[doc(hidden)]
    /// Assigns a device by file in **read-write** mode to a `Fdisk`.
    pub(crate) fn assign_device_by_file_read_write<T>(
        &mut self,
        device_file: File,
        device_path: T,
    ) -> Result<(), FdiskError>
    where
        T: AsRef<Path>,
    {
        log::debug!(
            "Fdisk::assign_device_by_file_read_write assigning read-write device: {:?}",
            device_path.as_ref()
        );

        Self::assign_device_by_file(self, device_file, device_path, 0)
    }

    #[doc(hidden)]
    /// Sets how partitions are addressed, i.e. by `sector` or `cylinder`.
    pub(crate) fn set_device_addressing(
        &mut self,
        addressing: DeviceAddressing,
    ) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::set_device_addressing setting device addressing unit: {:?}",
            addressing
        );

        let addressing_cstr = addressing.to_c_string();

        let result = unsafe { libfdisk::fdisk_set_unit(self.inner, addressing_cstr.as_ptr()) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::set_device_addressing set device addressing unit to: {:?}",
                    addressing
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set device addressing unit to: {:?}", addressing);
                log::debug!("Fdisk::set_device_addressing {}. libfdisk::fdisk_set_unit returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Enables/disables prompts for disk partitioning.
    fn display_dialogs(ptr: &mut Self, enable: bool) -> Result<(), FdiskError> {
        let op_str = if enable {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        // caution: value is reversed since function it is used on disables instead of enables
        // dialogs.
        let op = if enable { 0 } else { 1 };

        let result = unsafe { libfdisk::fdisk_disable_dialogs(ptr.inner, op) };

        match result {
            0 => {
                log::debug!("Fdisk::display_dialogs {}d dialogs", op_str);

                Ok(())
            }
            code => {
                let err_msg = format!("failed to {} dialogs", op_str);
                log::debug!("Fdisk::display_dialogs {}. libfdisk::fdisk_disable_dialogs returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Enable disk partitioning prompts.
    pub(crate) fn enable_interactive(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::disable_dialogs disabling partitioning prompts");

        Self::display_dialogs(self, true)
    }

    #[doc(hidden)]
    /// Disable disk partitioning prompts.
    pub(crate) fn disable_interactive(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::disable_dialogs disabling partitioning prompts");

        Self::display_dialogs(self, false)
    }

    #[doc(hidden)]
    /// Enables/disables display of partition details.
    fn display_partition_details(ptr: &mut Self, display: bool) -> Result<(), FdiskError> {
        let op_str = if display {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        let op = if display { 1 } else { 0 };

        let result = unsafe { libfdisk::fdisk_enable_details(ptr.inner, op) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::display_partition_details {}d display of partition details",
                    op_str
                );

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "Fdisk::display_partition_details failed to {} display of partition details",
                    op_str
                );
                log::debug!("Fdisk::display_partition_details {}. libfdisk::fdisk_enable_details returned error code: {:}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Shows each partition's detailed metadata when printing on the console.
    pub(crate) fn enable_partition_details(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::enable_partition_details enabling display of partition details");

        Self::display_partition_details(self, true)
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Hides each partition's detailed metadata when printing on the console.
    pub(crate) fn disable_partition_details(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::disable_partition_details disabling display of partition details");

        Self::display_partition_details(self, false)
    }

    #[doc(hidden)]
    /// Enables/disables display of partition lists without details.
    fn display_partitions_as_list(ptr: &mut Self, display: bool) -> Result<(), FdiskError> {
        let op_str = if display {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        let op = if display { 1 } else { 0 };

        let result = unsafe { libfdisk::fdisk_enable_listonly(ptr.inner, op) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::display_partitions_as_list {}d display of partition list only",
                    op_str
                );

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "Fdisk::display_partitions_as_list failed to {} display of partition list only",
                    op_str
                );
                log::debug!("Fdisk::display_partitions_as_list {}. libfdisk::fdisk_enable_listonly returned error code: {:}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Shows only a partition list when printing partition metadata on the console.
    pub(crate) fn enable_partition_list_only(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::enable_partition_list_only enabling display of partition list");

        Self::display_partitions_as_list(self, true)
    }

    #[allow(dead_code)]
    #[doc(hidden)]
    /// Stops showing only a partition list when printing partition metadata on the console.
    pub(crate) fn disable_partitions_as_list(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::disable_partitions_as_list disabling display of partition list");

        Self::display_partitions_as_list(self, false)
    }

    #[doc(hidden)]
    /// Sets the format in which to display partition sizes.
    pub(crate) fn set_partition_size_format(
        &mut self,
        format: SizeFormat,
    ) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::set_partition_size_format setting partition size formatting: {:?}",
            format
        );
        let c_format = format as u32 as i32;

        let result = unsafe { libfdisk::fdisk_set_size_unit(self.inner, c_format) };

        match result {
            0 => {
                log::debug!("Fdisk::set_partition_size_format set partition size display formatting to: {:?}", format);
                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to set partition size display formatting to: {:?}",
                    format
                );

                log::debug!("Fdisk::set_partition_size_format {}. libfdisk::fdisk_set_size_unit returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Enables/disables protection of the master boot record on a device.
    fn protect_data_on_first_sector(ptr: &mut Self, protect: bool) -> Result<(), FdiskError> {
        let op_str = if protect {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        let op = if protect { 1 } else { 0 };

        let result = unsafe { libfdisk::fdisk_enable_bootbits_protection(ptr.inner, op) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::protect_data_on_first_sector {}d protection for device's master boot record",
                    op_str
                );

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to {} protection for device's master boot record",
                    op_str
                );
                log::debug!("Fdisk::protect_data_on_first_sector {}. libfdisk::fdisk_enable_bootbits_protection returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Erases data on the assigned device's master boot record when creating a new partition table.
    pub(crate) fn erase_master_boot_record(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::erase_master_boot_record erasing data on device's master boot record");

        Self::protect_data_on_first_sector(self, false)
    }

    #[doc(hidden)]
    /// Protects data on the assigned device's master boot record when creating a new partition table.
    pub(crate) fn protect_master_boot_record(&mut self) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::protect_master_boot_record protecting data on device's master boot record"
        );

        Self::protect_data_on_first_sector(self, true)
    }

    #[doc(hidden)]
    /// Enables/disables device metadata erasure before writing a partition table to disk.
    fn wipe_metadata(ptr: &mut Self, wipe: bool) -> Result<(), FdiskError> {
        let op_str = if wipe {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        let op = if wipe { 1 } else { 0 };

        let result = unsafe { libfdisk::fdisk_enable_wipe(ptr.inner, op) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::wipe_metadata {}d erasure of device metadata",
                    op_str
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to {} erasure of device metadata", op_str);
                log::debug!("Fdisk::wipe_metadata {}. libfdisk::fdisk_enable_wipe returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Deletes all device metadata before writing a partition table to disk.
    pub(crate) fn enable_metadata_wipe(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::enable_metadata_wipe marking device metadata for erasure");

        Self::wipe_metadata(self, true)
    }

    #[doc(hidden)]
    /// Keeps all device metadata before writing a partition table to disk.
    pub(crate) fn disable_metadata_wipe(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::disable_metadata_wipe keeping device metadata");

        Self::wipe_metadata(self, false)
    }

    #[doc(hidden)]
    /// Overrides the values collected by the scanner run after a device is assigned to a
    /// `Fdisk`, then saves the new values.
    pub(crate) fn save_device_geometry_overrides(
        &mut self,
        cylinders: u32,
        heads: u32,
        sectors: u32,
    ) -> Result<(), FdiskError> {
        log::debug!("Fdisk::save_device_geometry_overrides saving device geometry overrides cylinders: {:?}, heads: {:?}, sectors: {:?} values", cylinders, heads, sectors);

        let result =
            unsafe { libfdisk::fdisk_save_user_geometry(self.inner, cylinders, heads, sectors) };

        match result {
            0 => {
                log::debug!("Fdisk::save_device_geometry_overrides saved device geometry overrides cylinders: {:?}, heads: {:?}, sectors: {:?} values", cylinders, heads, sectors);
                Ok(())
            }
            code => {
                let err_msg = format!("failed to save device geometry overrides cylinders: {:?}, heads: {:?}, sectors: {:?} values", cylinders, heads, sectors);
                log::debug!("Fdisk::save_device_geometry_overrides {}. libfdisk::fdisk_override_geometry returned error code: {:?}", err_msg, code);

                Err(FdiskError::Save(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Saves a value overriding the device's grain size. The device grain size is used to
    /// align partitions, and is by default equal to the optimal I/O size or 1 MiB, whichever is the
    /// largest.
    ///
    /// If the given `size` is too small, this `Fdisk` will use the largest value between the
    /// device's physical sector size and the minimum I/O size.
    pub(crate) fn save_device_grain_size_override(&mut self, size: u64) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::save_device_grain_size_override saving device grain size (bytes): {:?}",
            size
        );

        let result = unsafe { libfdisk::fdisk_save_user_grain(self.inner, size) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::save_device_grain_size_override saved device grain size (bytes): {:?}",
                    size
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to save device grain size (bytes): {:?}", size);
                log::debug!("Fdisk::save_device_grain_size_override {}. libfdisk::fdisk_save_user_grain returned error code: {:?}", err_msg, code);

                Err(FdiskError::Save(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Overrides the assigned device's logical and physical sectors sizes in bytes.
    pub(crate) fn save_device_sector_overrides(
        &mut self,
        physical_sector_size: u32,
        logical_sector_size: u32,
    ) -> Result<(), FdiskError> {
        log::debug!("Fdisk::save_device_sector_overrides saving sector size overrides (bytes) physical: {:?}, logical: {:?}", physical_sector_size, logical_sector_size);

        let result = unsafe {
            libfdisk::fdisk_save_user_sector_size(
                self.inner,
                physical_sector_size,
                logical_sector_size,
            )
        };

        match result {
            0 => {
                log::debug!("Fdisk::save_device_sector_overrides saved sector size overrides (bytes) physical: {:?}, logical: {:?}", physical_sector_size, logical_sector_size);

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to save sector size overrides (bytes) physical: {:?}, logical: {:?}",
                    physical_sector_size, logical_sector_size
                );
                log::debug!("Fdisk::save_device_sector_overrides {}. libfdisk::fdisk_save_user_sector_size returned error code: {:?}", err_msg, code);

                Err(FdiskError::Save(err_msg))
            }
        }
    }

    //---- END setters

    /// Creates a [`FdiskBuilder`] to configure and construct a new `Fdisk` instance.
    ///
    /// Call the `FdiskBuilder`'s [`build()`](crate::fdisk::FdiskBuilder::build) method to
    /// instantiate a new `Fdisk`.
    pub fn builder() -> FdiskBuilder {
        log::debug!("Fdisk::builder creating a new `FdiskBuilder` instance");

        CtxBuilder::builder()
    }

    //---- BEGIN mutators

    #[doc(hidden)]
    /// Closes the assigned device.
    fn close_assigned_device(ptr: &mut Self, no_sync: bool) -> Result<(), FdiskError> {
        let op_str = if no_sync {
            "without sync".to_owned()
        } else {
            "with sync".to_owned()
        };
        let op = if no_sync { 1 } else { 0 };

        let result = unsafe { libfdisk::fdisk_deassign_device(ptr.inner, op) };

        match result {
            0 => {
                log::debug!("Fdisk::close_assigned_device closed device {}", op_str);
                Ok(())
            }
            code => {
                let err_msg = format!("failed to close device {}", op_str);
                log::debug!("Fdisk::close_assigned_device {}. libfdisk::fdisk_deassign_device returned error code: {:?}", err_msg, code);

                Err(FdiskError::CloseDevice(err_msg))
            }
        }
    }

    /// Closes the device assigned at creation, makes sure all buffered data associated with the
    /// device's open file descriptor are saved.
    ///
    /// This method **waits** for all data to be written to the device before returning.
    pub fn close_device(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::close_device closing assigned device");

        Self::close_assigned_device(self, false)
    }

    /// Closes the device assigned at creation, makes sure all buffered data associated with the
    /// device's open file descriptor are saved.
    ///
    /// This method **does NOT wait** for all data to be written to the device before returning.
    pub fn close_device_async(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::close_device_async closing assigned device skipping sync");

        Self::close_assigned_device(self, true)
    }

    /// Discards all in-memory changes to this `Fdisk`, no data is written to the assigned device.
    ///
    /// Use this method if this `Fdisk` is in an undefined state after a major adverse event.
    pub fn discard_changes(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::discard_changes discarding changes");

        let result = unsafe { libfdisk::fdisk_reassign_device(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::discard_changes discarded changes");

                Ok(())
            }
            code => {
                let err_msg = "failed to discard changes".to_owned();
                log::debug!("Fdisk::discard_changes {}. libfdisk::fdisk_reassign_device returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    /// Forces the kernel to reread metadata about partitions in the partition table on the assigned device.
    pub fn reread_partition_entries(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::reread_partition_entries rereading partitions in partition table");

        let result = unsafe { libfdisk::fdisk_reread_partition_table(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::reread_partition_entries reread partitions in partition table");

                Ok(())
            }
            code => {
                let err_msg = "failed to reread partitions in partition table".to_owned();
                log::debug!("Fdisk::reread_partition_entries {}. libfdisk::fdisk_reread_partition_entries returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    /// Restores changed in-memory partition entries in the partition table to the same state as the one in the
    /// `entries_on_disk` parameter.
    ///
    /// **Note:** this function does not force the kernel to reread the whole partition table.
    /// Therefore, unmodified partitions can be mounted while this method operates.
    pub fn reread_changed_partition_entries(
        &mut self,
        entries_on_disk: &PartitionList,
    ) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::reread_changed_partition_entries rereading changed partition table entries"
        );

        let result = unsafe { libfdisk::fdisk_reread_changes(self.inner, entries_on_disk.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::reread_changed_partition_entries reread changed partition table entries");

                Ok(())
            }
            code => {
                let err_msg = "failed to reread changed partition table entries".to_owned();
                log::debug!("Fdisk::reread_changed_partition_entries {}. libfdisk::fdisk_reread_changes returned error code: {:?}", err_msg, code);

                Err(FdiskError::Restore(err_msg))
            }
        }
    }

    /// Sets the location of the first logical sector on disk.
    ///
    /// **Warning:** This is a very low-level function, use it only when you work with unusual
    /// partition tables like `GPT Protective MBR`, or hybrid partition tables on bootable media
    /// where the first partition may be located at a peculiar offset. It is **strongly**
    /// recommended to stick to the library's default settings.
    ///
    /// **Note:** The location of the first logical sector is always reset to the library's defaults
    /// after calling [`Fdisk::override_device_geometry`] or
    /// [`Fdisk::restore_default_lba_alignment`].
    ///
    /// **Caution:** This function modifies the in-memory partition table only, it does NOT update
    /// on-disk values. For example, a GPT Header contains `FirstUsableLBA` and `LastUsableLBA`
    /// fields that will not be updated.
    pub fn device_set_first_lba(&mut self, address: u64) -> Result<(), FdiskError> {
        log::debug!("Fdisk::device_set_first_lba setting first logical block address");

        let result = unsafe { libfdisk::fdisk_set_first_lba(self.inner, address) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::device_set_first_lba set first logical block address at: {:?}",
                    address
                );

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to set first logical block address at: {:?}",
                    address
                );
                log::debug!("Fdisk::device_set_first_lba {}. libfdisk::fdisk_set_first_lba returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    /// Sets the location of the last logical sector on disk.
    ///
    /// This value is equal, by default, to the number of sectors available on the assigned device,
    /// but may be adjusted per partition table. For example `GPT` partition tables keep a backup
    /// Header at the end of the disk which reduces the total number of sectors available.
    ///
    /// **Warning:** It is **strongly** recommended to stick to the library's default settings.
    ///
    /// **Note:** The location of the last logical sector is always reset to the library's defaults
    /// after calling [`Fdisk::override_device_geometry`] or
    /// [`Fdisk::restore_default_lba_alignment`].
    pub fn device_set_last_lba(&mut self, address: u64) -> Result<(), FdiskError> {
        log::debug!("Fdisk::device_set_last_lba setting last logical block address");

        let result = unsafe { libfdisk::fdisk_set_last_lba(self.inner, address) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::device_set_last_lba set last logical block address at: {:?}",
                    address
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set last logical block address at: {:?}", address);
                log::debug!("Fdisk::device_set_last_lba {}. libfdisk::fdisk_set_last_lba returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Align the LBA address to multiple of the device grain size.
    fn align_lba(fdisk: &mut Self, address: u64, direction: LBAAlign) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::align_lba aligning LBA {} to address: {:?}",
            direction,
            address
        );

        let result = unsafe { libfdisk::fdisk_align_lba(fdisk.inner, address, direction.into()) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::align_lba aligned LBA {} to address: {:?}",
                    direction,
                    address
                );

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to align LBA {} to address: {:?}",
                    direction, address
                );
                log::debug!(
                    "Fdisk::align_lba {}. libfdisk::fdisk_align_lba returned error code: {:?}",
                    err_msg,
                    code
                );

                Err(FdiskError::DataAlignment(err_msg))
            }
        }
    }

    /// Aligns the LBA to the next block/sector boundary.
    ///
    /// If the assigned device uses an alignment offset, the LBA is placed on the next physical
    /// sector boundary.
    pub fn align_lba_up(&mut self, address: u64) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::align_lba_up aligning LBA up to the address: {:?}",
            address
        );

        Self::align_lba(self, address, LBAAlign::Up)
    }

    /// Aligns the LBA to the previous block/sector boundary.
    ///
    /// If the assigned device uses an alignment offset, the LBA is placed on the previous physical
    /// sector boundary.
    pub fn align_lba_down(&mut self, address: u64) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::align_lba_down aligning LBA down to the address: {:?}",
            address
        );

        Self::align_lba(self, address, LBAAlign::Down)
    }

    /// Aligns the LBA to the nearest block/sector boundary.
    ///
    /// If the assigned device uses an alignment offset, the LBA is placed on the nearest physical
    /// sector boundary.
    pub fn align_lba_nearest(&mut self, address: u64) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::align_lba_nearest aligning LBA nearest to the address: {:?}",
            address
        );

        Self::align_lba(self, address, LBAAlign::Nearest)
    }

    /// Returns the value of the aligned LBA address in the given sector range.
    pub fn align_lba_in_range(&mut self, lba: u64, lower_bound: u64, upper_bound: u64) -> u64 {
        let address = unsafe {
            libfdisk::fdisk_align_lba_in_range(self.inner, lba, lower_bound, upper_bound)
        };
        log::debug!("Fdisk::align_lba_in_range address: {:?}", address);

        address
    }

    /// Temporarily overrides the assigned device's geometry. Call the
    /// [`Fdisk::restore_device_properties`] method to reset this `Fdisk` to its initial values.
    pub fn override_device_geometry(
        &mut self,
        cylinders: u32,
        heads: u32,
        sectors: u32,
    ) -> Result<(), FdiskError> {
        log::debug!("Fdisk::override_device_geometry overriding device geometry with new cylinders: {:?}, heads: {:?}, sectors: {:?} values", cylinders, heads, sectors);

        let result =
            unsafe { libfdisk::fdisk_override_geometry(self.inner, cylinders, heads, sectors) };

        match result {
            0 => {
                log::debug!("Fdisk::override_device_geometry overrode device geometry with new cylinders: {:?}, heads: {:?}, sectors: {:?} values", cylinders, heads, sectors);
                Ok(())
            }
            code => {
                let err_msg = format!("failed to override device geometry with new cylinders: {:?}, heads: {:?}, sectors: {:?} values", cylinders, heads, sectors);
                log::debug!("Fdisk::override_device_geometry {}. libfdisk::fdisk_override_geometry returned error code: {:?}", err_msg, code);

                Err(FdiskError::Override(err_msg))
            }
        }
    }

    /// Resets LBA alignment to its default value (specific to each type of partition table).
    pub fn restore_default_lba_alignment(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::restore_default_lba_alignment restoring default LBA alignment");

        let result = unsafe { libfdisk::fdisk_reset_alignment(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::restore_default_lba_alignment restored default LBA alignment");

                Ok(())
            }
            code => {
                let err_msg = "failed to restore default LBA alignment".to_owned();
                log::debug!("Fdisk::restore_default_lba_alignment {}. libfdisk::fdisk_reset_alignment returned error code: {:?}", err_msg, code);

                Err(FdiskError::Restore(err_msg))
            }
        }
    }

    /// Restores LBA alignment, device geometry, grain size, and sector sizes. The method rereads
    /// values from metadata on the assigned device, then applies the property overrides set by
    /// [`FdiskBuilder`], if any.
    pub fn restore_device_properties(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::restore_device_properties resetting device properties");

        let result = unsafe { libfdisk::fdisk_reset_device_properties(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::restore_device_properties reset device properties");

                Ok(())
            }
            code => {
                let err_msg = "failed to reset device properties".to_owned();
                log::debug!("Fdisk::restore_device_properties {}. libfdisk::fdisk_restore_device_properties returned error code: {:?}", err_msg, code);

                Err(FdiskError::Restore(err_msg))
            }
        }
    }

    /// Prints an `info`-level log message to the console.
    pub fn log_info<T>(&mut self, message: T) -> Result<(), FdiskError>
    where
        T: AsRef<str>,
    {
        let message = message.as_ref();
        log::debug!("Fdisk::log_info printing info message: {:?}", message);

        let msg_cstr = ffi_utils::as_ref_str_to_c_string(message)?;
        let fmt = CString::new("%s").unwrap();

        let result = unsafe { libfdisk::fdisk_info(self.inner, fmt.as_ptr(), msg_cstr.as_ptr()) };

        match result {
            0 => {
                log::debug!("Fdisk::log_info printed info message");

                Ok(())
            }
            code => {
                let err_msg = format!("failed to print info message: {:?}", message);
                log::debug!(
                    "Fdisk::log_info {}. libfdisk::fdisk_info returned error code: {:?}",
                    err_msg,
                    code
                );

                Err(FdiskError::Log(err_msg))
            }
        }
    }

    /// Prints a `warning`-level log message to the console.
    pub fn log_warn<T>(&mut self, message: T) -> Result<(), FdiskError>
    where
        T: AsRef<str>,
    {
        let message = message.as_ref();
        log::debug!("Fdisk::log_warn printing warning message: {:?}", message);

        let msg_cstr = ffi_utils::as_ref_str_to_c_string(message)?;
        let fmt = CString::new("%s").unwrap();

        let result = unsafe { libfdisk::fdisk_warnx(self.inner, fmt.as_ptr(), msg_cstr.as_ptr()) };

        match result {
            0 => {
                log::debug!("Fdisk::log_warn printed warning message");

                Ok(())
            }
            code => {
                let err_msg = format!("failed to print warning message: {:?}", message);
                log::debug!(
                    "Fdisk::log_warn {}. libfdisk::fdisk_warnx returned error code: {:?}",
                    err_msg,
                    code
                );

                Err(FdiskError::Log(err_msg))
            }
        }
    }

    /// Prints a `warning`-level log message to the console, and sets the C error code `errno`.
    pub fn log_warn_set_errno<T>(&mut self, message: T, errno: i32) -> Result<(), FdiskError>
    where
        T: AsRef<str>,
    {
        let message = message.as_ref();
        log::debug!(
            "Fdisk::log_warn_set_errno printing warning message: {:?} and setting errno to : {:?}",
            message,
            errno
        );

        let msg_cstr = ffi_utils::as_ref_str_to_c_string(message)?;
        let fmt = CString::new("%s").unwrap();

        let result =
            unsafe { libfdisk::fdisk_warn(self.inner, fmt.as_ptr(), msg_cstr.as_ptr(), errno) };

        match result {
            0 => {
                log::debug!("Fdisk::log_warn_set_errno printed warning message and set `errno`");

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to print warning message: {:?} and set `errno` to: {:?}",
                    message, errno
                );
                log::debug!(
                    "Fdisk::log_warn_set_errno {}. libfdisk::fdisk_warn returned error code: {:?}",
                    err_msg,
                    code
                );

                Err(FdiskError::Log(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Adds a new partition to the partition table to be created by this `Fdisk`.
    fn add_partition(
        ptr: *mut libfdisk::fdisk_context,
        partition_ptr: *mut libfdisk::fdisk_partition,
    ) -> Result<usize, FdiskError> {
        let mut partition_number_ptr = MaybeUninit::<libc::size_t>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_add_partition(ptr, partition_ptr, partition_number_ptr.as_mut_ptr())
        };

        match result {
            0 => {
                let partition_number = unsafe { partition_number_ptr.assume_init() };
                log::debug!(
                    "Fdisk::add_partition added new partition numbered: {:?}",
                    partition_number
                );
                Ok(partition_number)
            }
            code => {
                let err_msg = "failed to add new partition".to_owned();
                log::debug!("Fdisk::add_partition {}. libfdisk::fdisk_add_partition returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    /// Adds a new partition to the in-memory partition table held by this `Fdisk`. This operation
    /// is **non-interactive**, and uses the given `partition` parameter as a template.
    ///
    /// Using a template is particularly important for adding a partition to an `MBR` partition
    /// table. By setting a specific starting sector and/or partition number for the template, it
    /// is possible to differentiate between logical and extended partitions following the rules
    /// outlined below:
    ///
    /// - if the starting sector of `partition` is within the range reserved for the extended
    /// partition, this method will add a logical partition to the `MBR`,
    /// - if the starting sector of `partition` is outside the range reserved for the extended
    /// partition, this method will add a primary partition to the `MBR`,
    /// - if `partition` has a partition number < 4, this method will add a primary partition to
    /// the `MBR`,
    /// - if `partition` has a partition number >= 4, this method will add a logical partition to
    /// the `MBR`.
    ///
    /// If the template lacks essential information necessary to complete the process, it will
    /// revert to interactively asking for the missing data.
    pub fn partition_add(&mut self, partition: Partition) -> Result<usize, FdiskError> {
        log::debug!("Fdisk::partition_add adding a new partition");

        Self::add_partition(self.inner, partition.inner)
    }

    /// Adds a new partition to the partition table to be created by this `Fdisk`. This
    /// operation is **interactive**, using [`Prompt`](crate::core::prompt::Prompt)s to collect the
    /// partition's parameters.
    pub fn partition_add_interactive(&mut self) -> Result<usize, FdiskError> {
        log::debug!("Fdisk::partition_add adding a new partition (interactive)");

        Self::add_partition(self.inner, std::ptr::null_mut())
    }

    /// Appends the elements of the given [`PartitionList`] to this `Fdisk`'s in-memory partition table.
    ///
    /// **Note:** this method will ignore any [`Partition`] that does not use the first free starting
    /// sector, or lacks one.
    pub fn partitions_append(&self, partitions: PartitionList) -> Result<(), FdiskError> {
        log::debug!("Fdisk::partitions_append appending partitions to the partition table");

        unsafe {
            match libfdisk::fdisk_apply_table(self.inner, partitions.inner) {
                0 => {
                    log::debug!(
                        "Fdisk::partitions_append appended partitions to the partition table"
                    );

                    Ok(())
                }
                code => {
                    let err_msg = "failed to append partitions to the partition table".to_owned();
                    log::debug!("Fdisk::partitions_append {}. libfdisk::fdisk_apply_table returned error code: {:?}", err_msg, code);

                    Err(FdiskError::Config(err_msg))
                }
            }
        }
    }

    /// Deletes a partition with the given identification number from the partition table on the
    /// device assigned to this `Fdisk`.
    pub fn partition_delete(&mut self, partition_number: usize) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::partition_delete deleting partition with number: {:?}",
            partition_number
        );

        let result = unsafe { libfdisk::fdisk_delete_partition(self.inner, partition_number) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::partition_delete deleted partition with number: {:?}",
                    partition_number
                );
                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to delete partition with number: {:?}",
                    partition_number
                );
                log::debug!("Fdisk::partition_delete {}. libfdisk::fdisk_delete_partition returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    /// Deletes all partitions in the partition table on the device assigned to this `Fdisk`.
    pub fn partition_delete_all(&mut self) -> Result<(), FdiskError> {
        log::debug!("Fdisk::partition_delete_all deleting all partitions");

        let result = unsafe { libfdisk::fdisk_delete_all_partitions(self.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::partition_delete_all deleted all partitions");

                Ok(())
            }
            code => {
                let err_msg = "failed to delete all partitions".to_owned();
                log::debug!("Fdisk::partition_delete_all {}. libfdisk::fdisk_delete_all_partitions returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Returns the next partition's number.
    fn next_partition_number(
        ptr: &mut Self,
        partition: *mut libfdisk::fdisk_partition,
    ) -> Result<usize, FdiskError> {
        let mut number = MaybeUninit::<libc::size_t>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_partition_next_partno(partition, ptr.inner, number.as_mut_ptr())
        };
        match result {
            0 => {
                let part_number = unsafe { number.assume_init() };
                log::debug!("Fdisk::next_partition_number value: {:?}", part_number);

                Ok(part_number)
            }
            1 => {
                let err_msg = "no partition number available".to_owned();
                log::debug!("Fdisk::next_partition_number {}", err_msg);

                Err(FdiskError::NoNextPartitionNumber(err_msg))
            }
            code if code == -libc::ERANGE => {
                let err_msg = "partition number out of range".to_owned();
                log::debug!("Fdisk::next_partition_number {}", err_msg);

                Err(FdiskError::ResultOutOfRange(err_msg))
            }
            code if code == -libc::EINVAL => {
                let err_msg = "unable to ask for next partition number".to_owned();
                log::debug!("Fdisk::next_partition_number {}", err_msg);

                Err(FdiskError::DialogsDisabled(err_msg))
            }
            code => {
                let err_msg = "failed to get next partition number".to_owned();
                log::debug!("Fdisk::next_partition_number {}. libfdisk::fdisk_partition_next_partno returned error code: {:?}", err_msg, code);

                Err(FdiskError::Prompt(err_msg))
            }
        }
    }

    /// Asks the user to specify the next partition's number.
    pub fn partition_ask_next_number(&mut self) -> Result<usize, FdiskError> {
        log::debug!("Fdisk::partition_ask_next_number asking for next partition number");

        Self::next_partition_number(self, std::ptr::null_mut())
    }

    /// Returns the first free partition number following the `partition` template's, provided a
    /// partition number was not set at creation; falls back to interactively asking the user for a
    /// partition number otherwise.
    pub fn partition_next_number(&mut self, partition: &Partition) -> Result<usize, FdiskError> {
        log::debug!("Fdisk::partition_next_number getting next partition number");

        Self::next_partition_number(self, partition.inner)
    }

    /// Overrides the configuration of the partition with identification number matching
    /// `partition_number` with the `template`'s parameters.
    pub fn partition_override_settings(
        &mut self,
        partition_number: usize,
        template: &Partition,
    ) -> Result<(), FdiskError> {
        log::debug!("Fdisk::partition_override_settings overriding partition settings");

        let result =
            unsafe { libfdisk::fdisk_set_partition(self.inner, partition_number, template.inner) };

        match result {
            0 => {
                log::debug!("Fdisk::partition_override_settings overrode partition settings");

                Ok(())
            }
            code => {
                let err_msg = "failed to override partition settings".to_owned();
                log::debug!("Fdisk::partition_override_settings {}. libfdisk::fdisk_set_partition returned error code: {:?}", err_msg, code);

                Err(FdiskError::Override(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Sets the `Partition` matching the identification number `partition_number` for metadata erasure.
    fn wipe_partition(
        ptr: *mut libfdisk::fdisk_context,
        partition_number: usize,
        enable: bool,
    ) -> Result<(), FdiskError> {
        let op_str = if enable {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        let op = if enable { 1 } else { 0 };

        let result = unsafe { libfdisk::fdisk_wipe_partition(ptr, partition_number, op) };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::wipe_partition {}d wipe for partition with number: {:?}",
                    op_str,
                    partition_number
                );

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to {} wipe for partition with number: {:?}",
                    op_str, partition_number
                );
                log::debug!("Fdisk::wipe_partition {}. libfdisk::fdisk_wipe_partition returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    /// Marks all metadata on the [`Partition`] matching the given `partition_number` for deletion.
    pub fn partition_wipe_activate(&mut self, partition_number: usize) -> Result<(), FdiskError> {
        log::debug!("Fdisk::partition_wipe_activate enabling partition wipe");

        Self::wipe_partition(self.inner, partition_number, true)
    }

    /// Marks all metadata on the [`Partition`] matching the given `partition_number` for
    /// preservation.
    pub fn partition_wipe_deactivate(&mut self, partition_number: usize) -> Result<(), FdiskError> {
        log::debug!("Fdisk::partition_wipe_deactivate enabling partition wipe");

        Self::wipe_partition(self.inner, partition_number, false)
    }

    //---- END mutators

    //---- BEGIN getters

    #[doc(hidden)]
    /// Gets the string representation of the unit in which numerical metadata is displayed.
    fn unit_str<'b>(context: *mut libfdisk::fdisk_context, multiplicity: i32) -> &'b str {
        log::debug!("Fdisk::unit_str getting display unit");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_get_unit(context, multiplicity));
        }

        let unit_ptr = unsafe { ptr.assume_init() };
        ffi_utils::const_char_array_to_str_ref(unit_ptr).unwrap()
    }

    /// Returns the string representation in singular form of the unit in which numerical metadata
    /// is displayed.
    pub fn displayed_unit_singular(&self) -> &str {
        let unit = Self::unit_str(self.inner, libfdisk::FDISK_SINGULAR);
        log::debug!("Fdisk::displayed_unit_singular value: {:?}", unit);

        unit
    }

    /// Returns the string representation in plural form of the unit in which numerical metadata
    /// is displayed.
    pub fn displayed_unit_plural(&self) -> &str {
        let unit = Self::unit_str(self.inner, libfdisk::FDISK_PLURAL);
        log::debug!("Fdisk::displayed_unit_plural value: {:?}", unit);

        unit
    }

    /// Returns a reference to the parent partitioner of this `Fdisk` when it is a nested
    /// partitioner.
    ///
    /// # Panics
    ///
    /// May panic if the underlying `libfdisk` parent C pointer differs from the cached parent Rust
    /// reference.
    pub fn parent_partitioner(&self) -> Option<&Fdisk> {
        log::debug!("Fdisk::parent_partitioner getting reference to parent `Fdisk`");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_context>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_get_parent(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("Fdisk::parent_partitioner got no parent partitioner. libfdisk::fdisk_get_parent returned a NULL pointer");
                None
            }
            parent_ptr => {
                log::debug!("Fdisk::parent_partitioner got parent partitioner");
                self._parent.map(|parent| {
                    if parent.inner == parent_ptr {
                        log::debug!("Fdisk::parent_partitioner parent partitioner matches cached value");

                        parent
                    } else {
                        log::debug!(
                            "Fdisk::parent_partitioner parent partitioner does NOT match cached value"
                        );

                        panic!("Fdisk::parent_partitioner parent partitioner does NOT match cached value");
                    }
                })
            }
        }
    }

    /// Returns the number of sectors per cylinder when this `Fdisk` is set to display data in
    /// cylinder units, or `1` if device addressing is set to [`DeviceAddressing::Sector`] (see:
    /// [`FdiskBuilder::device_addressing`]).
    pub fn sectors_per_cylinder(&self) -> u64 {
        let sectors = unsafe { libfdisk::fdisk_get_units_per_sector(self.inner) };
        log::debug!("Fdisk::sectors_per_cylinder value: {:?}", sectors);

        sectors as u64
    }

    /// Returns the offset in bytes between logical and physical sectors.
    ///
    /// For backward compatibility, the first logical sector on 4K sector devices does not have to
    /// start on a naturally aligned physical sector boundary.
    pub fn device_alignment_offset(&self) -> u64 {
        let offset = unsafe { libfdisk::fdisk_get_alignment_offset(self.inner) };
        log::debug!("Fdisk::device_alignment_offset value: {:?}", offset);

        offset
    }

    /// Returns the underlying file descriptor associated with the assigned device.
    ///
    /// # Safety
    ///
    /// You must guarantee that for the duration of this `Fdisk`'s lifetime, nobody will close the
    /// file descriptor returned by this method.
    pub unsafe fn device_borrow_fd(&self) -> BorrowedFd {
        log::debug!("Fdisk::device_borrow_fd borrowing assigned device's file descriptor");

        let raw_fd = unsafe { libfdisk::fdisk_get_devfd(self.inner) };

        unsafe { BorrowedFd::borrow_raw(raw_fd) }
    }

    /// Returns a device's model.
    pub fn device_model(&self) -> Option<&str> {
        log::debug!("Fdisk::device_model getting device model");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_get_devmodel(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            model_ptr if model_ptr.is_null() => {
                log::debug!("Fdisk::device_model got no device model. libfdisk::fdisk_get_devmodel returned a NULL pointer");

                None
            }
            model_ptr => {
                let dev_model = ffi_utils::const_char_array_to_str_ref(model_ptr).ok();
                log::debug!("Fdisk::device_model got device model: {:?}", dev_model);

                dev_model
            }
        }
    }

    /// Returns the assigned device's name.
    pub fn device_name(&self) -> Option<&Path> {
        log::debug!("Fdisk::device_name getting assigned device's name");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_get_devname(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to get assigned device's name".to_owned();
                log::debug!(
                    "Fdisk::device_name {}. libfdisk::fdisk_get_devname returned a NULL pointer",
                    err_msg
                );

                None
            }
            dev_name_ptr => {
                let dev_name = ffi_utils::const_c_char_array_to_path(dev_name_ptr);
                log::debug!(
                    "Fdisk::device_name got assigned device name: {:?}",
                    dev_name
                );

                Some(dev_name)
            }
        }
    }

    /// Returns the assigned device's identification number (`0` for an image file).
    pub fn device_number(&self) -> u64 {
        let dev_num = unsafe { libfdisk::fdisk_get_devno(self.inner) };
        log::debug!("Fdisk::device_number value: {:?}", dev_num);

        dev_num
    }

    /// Returns the location of the first logical block available for creating a new partition on
    /// the assigned device.
    pub fn device_first_lba(&self) -> u64 {
        let first_lba = unsafe { libfdisk::fdisk_get_first_lba(self.inner) };
        log::debug!(
            "Fdisk::device_first_lba address of first logical block: {:?}",
            first_lba
        );

        first_lba
    }

    /// Returns the location of the last logical block available on the assigned device.
    pub fn device_last_lba(&self) -> u64 {
        let last_lba = unsafe { libfdisk::fdisk_get_last_lba(self.inner) };
        log::debug!(
            "Fdisk::device_last_lba address of last logical block: {:?}",
            last_lba
        );

        last_lba
    }

    /// Returns the number of cylinder subdivisions of the assigned device.
    pub fn device_count_cylinders(&self) -> u64 {
        let cylinders = unsafe { libfdisk::fdisk_get_geom_cylinders(self.inner) };
        log::debug!(
            "Fdisk::device_count_cylinders number of cylinders: {:?}",
            cylinders
        );

        cylinders
    }

    /// Returns the number of read-and-write heads (i.e. tracks per cylinder) of the assigned
    /// device.
    pub fn device_count_heads(&self) -> u64 {
        let heads = unsafe { libfdisk::fdisk_get_geom_heads(self.inner) };
        log::debug!("Fdisk::device_count_heads number of heads: {:?}", heads);

        heads as u64
    }

    /// Returns the number of sectors per track of the assigned device.
    pub fn device_count_sectors(&self) -> u64 {
        let sectors = unsafe { libfdisk::fdisk_get_geom_sectors(self.inner) };
        log::debug!(
            "Fdisk::device_count_sectors number of sectors per track: {:?}",
            sectors
        );

        sectors
    }

    /// Returns the device's grain size in bytes (usually `1 MiB`).
    pub fn device_grain_size(&self) -> u64 {
        let size = unsafe { libfdisk::fdisk_get_grain_size(self.inner) };
        log::debug!("Fdisk::device_grain_size value: {:?}", size);

        size
    }

    /// Returns the preferred minimum number of bytes for random Input/Output on the assigned
    /// device.
    pub fn device_minimum_io_size(&self) -> u64 {
        let min_io_size = unsafe { libfdisk::fdisk_get_minimal_iosize(self.inner) };
        log::debug!(
            "Fdisk::device_minimum_io_size minimal I/O size: {:?}",
            min_io_size
        );

        min_io_size
    }

    /// Returns the preferred optimal number of bytes for streaming Input/Output on the assigned
    /// device.
    pub fn device_optimal_io_size(&self) -> u64 {
        let opt_io_size = unsafe { libfdisk::fdisk_get_minimal_iosize(self.inner) };
        log::debug!(
            "Fdisk::device_optimal_io_size optimal I/O size: {:?}",
            opt_io_size
        );

        opt_io_size
    }

    /// Returns the size of the assigned device in bytes.
    pub fn device_size_in_bytes(&self) -> u64 {
        let size = self.device_size_in_sectors() * self.device_bytes_per_logical_sector();
        log::debug!("Fdisk::device_size_in_bytes size (bytes): {:?}", size);

        size
    }

    /// Returns the size of the assigned device in logical sectors.
    pub fn device_size_in_sectors(&self) -> u64 {
        let size = unsafe { libfdisk::fdisk_get_nsectors(self.inner) };
        log::debug!("Fdisk::device_size_in_sectors size (sectors): {:?}", size);

        size
    }

    /// Returns the size in bytes of a logical sector.
    pub fn device_bytes_per_logical_sector(&self) -> u64 {
        let size = unsafe { libfdisk::fdisk_get_sector_size(self.inner) };
        log::debug!(
            "Fdisk::device_bytes_per_logical_sector bytes per sector: {:?}",
            size
        );

        size
    }

    /// Returns the size in bytes of a physical sector.
    pub fn device_bytes_per_physical_sector(&self) -> u64 {
        let size = unsafe { libfdisk::fdisk_get_physector_size(self.inner) };
        log::debug!(
            "Fdisk::device_bytes_per_physical_sector bytes per sector: {:?}",
            size
        );

        size
    }

    /// Returns `true` if the caller answers `yes` to the `question`.
    pub fn ask_yes_no_question<T>(&self, question: T) -> Result<bool, FdiskError>
    where
        T: AsRef<str>,
    {
        let question = question.as_ref();
        let question_cstr = ffi_utils::as_ref_str_to_c_string(question)?;
        log::debug!(
            "Fdisk::ask_yes_no_question asking yes/no question: {:?}",
            question
        );

        let mut answer = MaybeUninit::<libc::c_int>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_ask_yesno(self.inner, question_cstr.as_ptr(), answer.as_mut_ptr())
        };

        match result {
            0 => {
                let answer = unsafe { answer.assume_init() };
                let answer_str = if answer == 1 {
                    "yes".to_owned()
                } else {
                    "no".to_owned()
                };
                log::debug!("Fdisk::ask_yes_no_question got answer: {:?}", answer_str);

                Ok(answer == 1)
            }
            code => {
                let err_msg = format!("failed to get answer to question: {:?}", question);
                log::debug!("Fdisk::ask_yes_no_question {}. libfdisk::fdisk_ask_yesno returned error code: {:?}", err_msg, code);

                Err(FdiskError::Prompt(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Asks the user for a partition number on the console.
    fn request_partition_number(
        ptr: *mut libfdisk::fdisk_context,
        want_new: libc::c_int,
    ) -> Result<usize, FdiskError> {
        let op_str = if want_new == 0 {
            "used".to_owned()
        } else {
            "unused".to_owned()
        };
        log::debug!(
            "Fdisk::request_partition_number requesting {} partition number",
            op_str
        );

        let mut partition_number = MaybeUninit::<usize>::zeroed();

        let result =
            unsafe { libfdisk::fdisk_ask_partnum(ptr, partition_number.as_mut_ptr(), want_new) };

        match result {
            0 => {
                let partition_number = unsafe { partition_number.assume_init() };
                log::debug!(
                    "Fdisk::request_partition_number got {} partition number: {:?}",
                    op_str,
                    partition_number
                );

                Ok(partition_number)
            }
            1 => {
                let err_msg = format!("no {} partition number available", op_str);
                log::debug!("Fdisk::request_partition_number {}", err_msg);

                Err(FdiskError::Prompt(err_msg))
            }
            code => {
                let err_msg = match -code {
                    libc::ENOMEM => "out of memory".to_owned(),
                    _ => format!("failed to request {} partition number", op_str),
                };

                log::debug!("Fdisk::request_partition_number {}. libfdisk::fdisk_ask_partnum returned error code: {:?}", err_msg, code);

                Err(FdiskError::Prompt(err_msg))
            }
        }
    }

    /// Asks the caller for a used partition number.
    pub fn ask_partition_number_used(&self) -> Result<usize, FdiskError> {
        Self::request_partition_number(self.inner, 0)
    }

    /// Asks the caller for an unused partition number.
    pub fn ask_partition_number_unused(&self) -> Result<usize, FdiskError> {
        Self::request_partition_number(self.inner, 1)
    }

    /// Prompts the caller for a numerical value.
    pub fn ask_number_in_range<T>(
        &self,
        question: T,
        default_value: u64,
        lower_bound: u64,
        upper_bound: u64,
    ) -> Result<libfdisk::uintmax_t, FdiskError>
    where
        T: AsRef<str>,
    {
        log::debug!(
            "Fdisk::ask_number_in_range requesting value in range [{:?}, {:?}] (default: {:?})",
            lower_bound,
            upper_bound,
            default_value
        );
        let question = question.as_ref();
        let question_cstr = ffi_utils::as_ref_str_to_c_string(question)?;

        let mut ptr = MaybeUninit::<libfdisk::uintmax_t>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_ask_number(
                self.inner,
                lower_bound,
                default_value,
                upper_bound,
                question_cstr.as_ptr(),
                ptr.as_mut_ptr(),
            )
        };

        match result {
            0 => {
                let obtained = unsafe { ptr.assume_init() };
                log::debug!("Fdisk::ask_number_in_range got value: {:?}", obtained);

                Ok(obtained)
            }
            code => {
                let err_msg = match -code {
                    libc::ENOMEM => "out of memory".to_owned(),
                    _ => format!(
                        "error while requesting value in range [{:?}, {:?}] (default: {:?})",
                        lower_bound, upper_bound, default_value
                    ),
                };

                log::debug!("Fdisk::ask_number_in_range {}. libfdisk::fdisk_ask_number returned error code: {:?}", err_msg, code);

                Err(FdiskError::Prompt(err_msg))
            }
        }
    }

    /// Prompts the caller for a string value.
    pub fn ask_string_value<T>(&self, question: T) -> Result<String, FdiskError>
    where
        T: AsRef<str>,
    {
        log::debug!("Fdisk::ask_string_value requesting string value");
        let question = question.as_ref();
        let question_cstr = ffi_utils::as_ref_str_to_c_string(question)?;

        let mut ptr = MaybeUninit::<*mut libc::c_char>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_ask_string(self.inner, question_cstr.as_ptr(), ptr.as_mut_ptr())
        };

        match result {
            0 => {
                let answer_ptr = unsafe { ptr.assume_init() };
                let answer = ffi_to_string_or_empty!(answer_ptr);

                log::debug!("Fdisk::ask_string_value got string value: {:?}", answer);

                Ok(answer)
            }
            code => {
                let err_msg = match -code {
                    libc::ENOMEM => "out of memory".to_owned(),
                    _ => "failed to request a string value".to_owned(),
                };

                log::debug!("Fdisk::ask_string_value {}. libfdisk::fdisk_ask_string returned error code: {:?}", err_msg, code);

                Err(FdiskError::Prompt(err_msg))
            }
        }
    }

    /// Returns the name of the already existing file system or partition table detected.
    pub fn device_describe_collisions(&self) -> Option<&str> {
        log::debug!("Fdisk::device_describe_collisions describing metadata collisions");
        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_get_collision(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("Fdisk::device_describe_collisions found no metadata conflict. libfdisk::fdisk_get_collision returned a NULL pointer");

                None
            }
            desc_ptr => {
                let collisions = ffi_utils::const_char_array_to_str_ref(desc_ptr).ok();
                log::debug!(
                    "Fdisk::device_describe_collisions found collisions for: {:?}",
                    collisions
                );

                collisions
            }
        }
    }

    /// Returns the format in which a `Fdisk` displays partition sizes.
    pub fn partition_size_format(&self) -> SizeFormat {
        let code = unsafe { libfdisk::fdisk_get_size_unit(self.inner) };
        let size_format = SizeFormat::try_from(code as u32).unwrap();
        log::debug!("Fdisk::partition_size_format value: {:?}", size_format);

        size_format
    }

    /// Returns the assigned device's name.
    /// Returns the content of a [`Partition`]'s field in string form.
    pub fn partition_field_to_string(
        &self,
        field: Field,
        partition: &Partition,
    ) -> Result<String, FdiskError> {
        log::debug!(
            "Fdisk::partition_field_to_string convert content of partition field {:?} to string",
            field
        );
        let cfield = field as u32 as i32;

        let mut content_ptr = MaybeUninit::<*mut libc::c_char>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_partition_to_string(
                partition.inner,
                self.inner,
                cfield,
                content_ptr.as_mut_ptr(),
            )
        };

        match result {
            0 => {
                let ptr = unsafe { content_ptr.assume_init() };
                let field_content = ffi_to_string_or_empty!(ptr);
                log::debug!("Fdisk::partition_field_to_string converted content of partition field {:?} to {:?}", field, field_content);

                Ok(field_content)
            }
            code => {
                let err_msg = format!("failed to convert content of partition field {:?}", field);
                log::debug!("Fdisk::partition_field_to_string {}. libfdisk::fdisk_partition_to_string returned error code: {:?}", err_msg, code);

                match -code {
                    libc::ENOMEM => Err(FdiskError::OutOfMemory(err_msg)),
                    _ => Err(FdiskError::Unexpected(err_msg)),
                }
            }
        }
    }

    #[doc(hidden)]
    /// Gets a partition by its index number in a partition array.
    fn get_partition_by_number(
        fdisk: &Self,
        partition_number: usize,
    ) -> Option<*mut libfdisk::fdisk_partition> {
        let mut partition_ptr = MaybeUninit::<*mut libfdisk::fdisk_partition>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_get_partition(fdisk.inner, partition_number, partition_ptr.as_mut_ptr())
        };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::get_partition_by_number got partition with identification number: {:?}",
                    partition_number
                );

                let partition = unsafe { partition_ptr.assume_init() };

                Some(partition)
            }
            code => {
                let err_msg = format!(
                    "no partition with identification number: {:?}",
                    partition_number
                );
                log::debug!("Fdisk::get_partition_by_number {}. libfdisk::fdisk_get_partition returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    /// Returns a reference to a [`Partition`] from its identification number.
    pub fn partition_by_number(&self, partition_number: usize) -> Option<&Partition> {
        log::debug!(
            "Partition::partition_by_number getting partition with identification number: {:?}",
            partition_number
        );

        Self::get_partition_by_number(self, partition_number)
            .map(|ptr| owning_ref_from_ptr!(self, Partition, ptr))
    }

    /// Returns a mutable reference to a [`Partition`] from its identification number.
    pub fn partition_by_number_mut(&mut self, partition_number: usize) -> Option<&mut Partition> {
        log::debug!(
            "Partition::partition_by_number_mut getting partition with identification number: {:?}",
            partition_number
        );

        Self::get_partition_by_number(self, partition_number)
            .map(|ptr| owning_mut_from_ptr!(self, Partition, ptr))
    }

    /// Returns a list of unallocated spaces on the assigned device as a collection of
    /// [`Partition`]s, or `None` if the device has no partition table.
    ///
    /// **Note:** this method will ignore free space smaller than the assigned device's alignment
    /// unit (see [`Fdisk::device_alignment_unit`])
    pub fn list_empty_spaces(&self) -> Option<PartitionList> {
        log::debug!("Fdisk::list_empty_spaces listing unallocated spaces");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_table>::zeroed();

        let result = unsafe { libfdisk::fdisk_get_freespaces(self.inner, ptr.as_mut_ptr()) };

        match result {
            0 => {
                log::debug!("Fdisk::list_empty_spaces listed unallocated spaces");
                let ptr = unsafe { ptr.assume_init() };
                let list = PartitionList::from_ptr(ptr);

                Some(list)
            }
            code => {
                let err_msg = "failed to list unallocated spaces on assigned device".to_owned();
                log::debug!("Fdisk::list_empty_spaces {}. libfdisk::fdisk_get_freespaces returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    /// Returns a list of the [`Partition`]s in this `Fdisk`.
    pub fn list_partitions(&self) -> Option<PartitionList> {
        log::debug!("Fdisk::list_partitions extracting partitions from partition table");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_table>::zeroed();

        let result = unsafe { libfdisk::fdisk_get_partitions(self.inner, ptr.as_mut_ptr()) };

        match result {
            0 => {
                log::debug!("Fdisk::list_partitions extracted partitions from partition table");
                let ptr = unsafe { ptr.assume_init() };
                let list = PartitionList::from_ptr(ptr);

                Some(list)
            }
            code => {
                let err_msg = match -code {
                    libc::EINVAL => "no partition table on device".to_owned(),
                    libc::ENOSYS => "no partition in partition table".to_owned(),
                    libc::ENOMEM => "out of memory".to_owned(),
                    _ => "failed to list partitions in partition table".to_owned(),
                };

                log::debug!("Fdisk::list_partitions {}. libfdisk::fdisk_get_partitions returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    //---- END getters

    //---- BEGIN predicates

    /// Returns `true` when the given `partition_number` is being used.
    ///
    /// **Note:** the first partition has identification number `0`.
    pub fn partition_is_number_in_use(&self, partition_number: usize) -> bool {
        let state = unsafe { libfdisk::fdisk_is_partition_used(self.inner, partition_number) == 1 };
        log::debug!("Fdisk::partition_is_number_in_use value: {:?}", state);

        state
    }

    /// Returns `true` when all metadata on the device area specified by the [`Partition`] will be
    /// wiped when the partition table is written to disk.
    pub fn is_partition_wipe_active(&self, partition: &Partition) -> bool {
        let state = unsafe { libfdisk::fdisk_partition_has_wipe(self.inner, partition.inner) == 1 };
        log::debug!("Fdisk::is_partition_wipe_active value: {:?}", state);

        state
    }

    /// Returns `true` if the user has overridden some device properties.
    pub fn has_overriden_device_properties(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_has_user_device_properties(self.inner) == 1 };
        log::debug!("Fdisk::has_overriden_device_properties value: {:?}", state);

        state
    }

    /// Returns `true` the `LBA` is aligned to a physical sector boundary.
    pub fn is_lba_physically_aligned(&self, lba: u64) -> bool {
        let state = unsafe { libfdisk::fdisk_lba_is_phy_aligned(self.inner, lba) == 1 };
        log::debug!("Fdisk::is_lba_physically_aligned value: {:?}", state);

        state
    }

    /// Returns `true` when this `Fdisk` is set to display each partition's detailed metadata when
    /// printing on the console.
    pub fn displays_partition_details(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_is_details(self.inner) == 1 };
        log::debug!("Fdisk::displays_partition_details value:{:?}", state);

        state
    }

    /// Returns `true` when this `Fdisk` is set to only display a partitions list **without**
    /// detailed metadata when printing on the console.
    pub fn displays_partition_list_only(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_is_listonly(self.inner) == 1 };
        log::debug!("Fdisk::displays_partition_list_only value: {:?}", state);

        state
    }

    /// Returns `true` when this `Fdisk` is set to display partition metadata in cylinder
    /// units.
    pub fn displays_metadata_in_cylinders(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_use_cylinders(self.inner) == 1 };
        log::debug!("Fdisk::displays_metadata_in_cylinders value: {:?}", state);

        state
    }

    /// Returns `true` when device partitioning is dialog-driven.
    pub fn is_partitioning_interactive(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_has_dialogs(self.inner) == 1 };
        log::debug!("Fdisk::is_partitioning_interactive value: {:?}", state);

        state
    }

    /// Returns `true` if this `Fdisk` is set to protect the master boot record of its assigned
    /// device when creating a new partition table.
    pub fn protects_master_boot_record(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_has_protected_bootbits(self.inner) == 1 };
        log::debug!("Fdisk::protects_master_boot_record value: {:?}", state);

        state
    }

    /// Returns `true` when this `Fdisk` erases all device metadata before writing a new
    /// partition table.
    pub fn wipes_device_metadata(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_has_wipe(self.inner) == 1 };
        log::debug!("Fdisk::wipes_device_metadata value: {:?}", state);

        state
    }

    /// Returns `true` if `libblkid` detects an already existing file system or partition table on
    /// the assigned device.
    ///
    /// **Note:** `libblkid` does not support all types of partition table, which can lead to some
    /// inconsistencies where [`Fdisk::device_has_partition_table`] returns `false` while this
    /// method returns `true`.
    pub fn device_has_collisions(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_is_ptcollision(self.inner) == 1 };
        log::debug!("Fdisk::device_has_collisions value: {:?}", state);

        state
    }

    /// Returns `true` when there is a partition table on the assigned device.
    pub fn device_has_partition_table(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_has_label(self.inner) == 1 };
        log::debug!("Fdisk::device_has_partition_table value: {:?}", state);

        state
    }

    /// Returns `true` when the assigned device is currently in use by the Operating System.
    ///
    /// **Warning:** always returns `false` if the device was assigned by file, or it is opened in read-only mode.
    pub fn device_is_in_use(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_device_is_used(self.inner) == 1 };
        log::debug!("Fdisk::device_is_in_use value: {:?}", state);

        state
    }

    /// Returns `true` when the assigned device is open in read-only mode.
    pub fn device_is_read_only(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_is_readonly(self.inner) == 1 };
        log::debug!("Fdisk::device_is_read_only value: {:?}", state);

        state
    }

    /// Returns `true` when the assigned device is an image file rather than a physical block device.
    pub fn device_is_image_file(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_is_regfile(self.inner) == 1 };
        log::debug!("Fdisk::device_is_image_file value: {:?}", state);

        state
    }

    //---- END predicates
}

impl<'a> AsRef<Fdisk<'a>> for Fdisk<'a> {
    #[inline]
    fn as_ref(&self) -> &Fdisk<'a> {
        self
    }
}

impl<'a> Drop for Fdisk<'a> {
    fn drop(&mut self) {
        log::debug!("Fdisk::drop deallocating `Fdisk` instance");

        unsafe { libfdisk::fdisk_unref_context(self.inner) }

        // Release heap allocated PartitionTable references.
        while let Some(gc_item) = self.gc.pop() {
            gc_item.destroy();
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::fdisk::DeviceAddressing;
    use crate::fdisk::SizeFormat;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    //---- Helper functions

    static BASE_DIR_TEST_IMG_FILES: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/third-party/vendor/util-linux/blkid/images"
    );

    static PATH_BLANK_IMG_FILE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/images/blank-10MB.img.xz"
    );

    fn decode_into<P, W>(xz_file_path: P, writer: &mut W) -> std::io::Result<u64>
    where
        P: AsRef<Path>,
        W: Write + ?Sized,
    {
        let xz_file_path = xz_file_path.as_ref();

        // Copy decompressed image to temporary file
        let compressed_image_file = std::fs::File::open(xz_file_path)?;
        let mut decompressed = xz2::read::XzDecoder::new(compressed_image_file);

        std::io::copy(&mut decompressed, writer)
    }

    /// Creates a named temporary image file with one of the supported file systems from the
    /// compressed samples.
    fn disk_image_with_fs(fs_type: &str) -> NamedTempFile {
        let img_path = format!("{BASE_DIR_TEST_IMG_FILES}/filesystems/{fs_type}.img.xz");
        let mut named_file = NamedTempFile::new().expect("failed to get new NamedTempFile");

        decode_into(img_path, named_file.as_file_mut()).expect("failed to create named disk image");
        named_file
    }

    /// Creates a temporary image file with one of the supported partition tables from the
    /// compressed samples.
    fn disk_image_with_pt(pt_type: &str) -> NamedTempFile {
        let img_path = format!("{BASE_DIR_TEST_IMG_FILES}/partition_tables/{pt_type}.img.xz");
        let mut named_file = NamedTempFile::new().expect("failed to get new NamedTempFile");

        decode_into(img_path, named_file.as_file_mut()).expect("failed to create named disk image");
        named_file
    }

    // Create a temporary 50MB blank image file.
    fn blank_image_file() -> NamedTempFile {
        let mut named_file = NamedTempFile::new().expect("failed to get new NamedTempFile");

        decode_into(PATH_BLANK_IMG_FILE, named_file.as_file_mut())
            .expect("failed to create named disk image");
        named_file
    }

    //-------------------------------------------------------------------------

    #[test]
    #[should_panic(
        expected = "one of the methods `assign_device` or `assign_device_by_file` must be called"
    )]
    fn fdisk_must_assign_a_device() {
        let _ = Fdisk::builder().build().unwrap();
    }

    #[test]
    #[should_panic(expected = "failed to assign read-only device")]
    fn fdisk_can_not_assign_a_regular_file_by_pathname() {
        let regular_file = NamedTempFile::new().unwrap();
        let _ = Fdisk::builder()
            .assign_device(regular_file.path())
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "failed to assign read-only device")]
    fn fdisk_can_not_assign_a_regular_file_by_file_stream() {
        let regular_file = NamedTempFile::new().unwrap();
        let (file, temp_path) = regular_file.into_parts();
        let _ = Fdisk::builder()
            .assign_device_by_file(file, temp_path.as_os_str())
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic(
        expected = "methods `display_partition_details` and `display_partition_list_only` can not be called at the same time"
    )]
    fn fdisk_can_not_both_display_details_and_list_only() {
        let tmp_image = disk_image_with_pt("gpt");
        let _ = Fdisk::builder()
            .assign_device(tmp_image.path())
            .display_partition_details()
            .display_partition_list_only()
            .build()
            .unwrap();
    }

    #[test]
    fn fdisk_can_assign_a_device_by_pathname_in_read_only_mode() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");

        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.device_is_read_only();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_assign_a_device_by_pathname_in_read_write_mode() -> crate::Result<()> {
        let tmp_image = disk_image_with_fs("ext4");

        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .enable_read_write()
            .build()?;

        let actual = disk.device_is_read_only();
        let expected = false;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_set_device_addressing() -> crate::Result<()> {
        let tmp_image = disk_image_with_fs("ext4");

        // Default:  DeviceAddressing::Sector
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.sectors_per_cylinder();
        let expected = 1;
        assert_eq!(actual, expected);

        let addressing = DeviceAddressing::Cylinder;
        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .device_addressing(addressing)
            .build()?;

        let actual = disk.sectors_per_cylinder();
        assert!(actual > 1);
        Ok(())
    }

    #[test]
    fn fdisk_can_enable_interactive_partitioning() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");

        // Default
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.is_partitioning_interactive();
        let expected = false;
        assert_eq!(actual, expected);

        // Enabled
        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .enable_interactive()
            .build()?;

        let actual = disk.is_partitioning_interactive();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_set_erase_master_boot_record() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");

        // Default
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.protects_master_boot_record();
        let expected = true;
        assert_eq!(actual, expected);

        // Enabled
        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .erase_master_boot_record()
            .build()?;

        let actual = disk.protects_master_boot_record();
        let expected = false;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_set_partition_size_format() -> crate::Result<()> {
        let tmp_image = blank_image_file();

        // Default
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.partition_size_format();
        let expected = SizeFormat::Bytes;
        assert_eq!(actual, expected);

        let format = SizeFormat::HumanReadable;
        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .partition_size_format(format)
            .build()?;

        let actual = disk.partition_size_format();
        let expected = format;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_set_wipe_device_metadata() -> crate::Result<()> {
        let tmp_image = blank_image_file();

        // Default
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.wipes_device_metadata();
        let expected = false;
        assert_eq!(actual, expected);

        // Enabled
        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .wipe_device_metadata()
            .build()?;

        let actual = disk.wipes_device_metadata();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_discard_changes() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");
        let mut disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let old_first_lba = disk.device_first_lba();

        disk.device_set_first_lba(64)?;

        let actual = disk.device_first_lba();
        let expected = 64;
        assert_eq!(actual, expected);

        disk.discard_changes()?;

        let actual = disk.device_first_lba();
        let expected = old_first_lba;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_get_display_units() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.displayed_unit_singular();
        let expected = "sector";
        assert_eq!(actual, expected);

        let actual = disk.displayed_unit_plural();
        let expected = "sectors";
        assert_eq!(actual, expected);

        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .device_addressing(DeviceAddressing::Cylinder)
            .build()?;

        let actual = disk.displayed_unit_singular();
        let expected = "cylinder";
        assert_eq!(actual, expected);

        let actual = disk.displayed_unit_plural();
        let expected = "cylinders";
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_get_sectors_per_cylinder() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.sectors_per_cylinder();
        let expected = 1;
        assert_eq!(actual, expected);

        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .device_addressing(DeviceAddressing::Cylinder)
            .build()?;

        let actual = disk.sectors_per_cylinder();
        let expected = disk.device_count_heads() * disk.device_count_sectors();
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_recognize_an_image_file() -> crate::Result<()> {
        let tmp_image = blank_image_file();

        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.device_is_image_file();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_recognize_an_image_file_with_or_without_a_partition_table() -> crate::Result<()> {
        let tmp_image = blank_image_file();
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.device_has_partition_table();
        let expected = false;
        assert_eq!(actual, expected);

        let tmp_image = disk_image_with_pt("gpt");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.device_has_partition_table();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_assign_a_device_by_file() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");
        let (file, temp_path) = tmp_image.into_parts();

        let disk = Fdisk::builder()
            .assign_device_by_file(file, temp_path.as_os_str())
            .build()?;

        let actual = disk.device_is_image_file();
        let expected = true;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_set_device_first_lba() -> crate::Result<()> {
        let tmp_image = blank_image_file();

        let mut disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        disk.device_set_first_lba(64)?;

        let actual = disk.device_first_lba();
        let expected = 64;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_set_device_last_lba() -> crate::Result<()> {
        let tmp_image = blank_image_file();
        let mut disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        disk.device_set_last_lba(64)?;

        let actual = disk.device_last_lba();
        let expected = 64;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_get_a_device_s_name() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.device_name();
        let expected = Some(tmp_image.path());
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_read_the_geometry_of_a_device() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.device_number();
        let expected = 0;
        assert_eq!(actual, expected);

        let actual = disk.device_count_cylinders();
        let expected = 1;
        assert_eq!(actual, expected);

        let actual = disk.device_count_heads();
        let expected = 255;
        assert_eq!(actual, expected);

        let actual = disk.device_count_sectors();
        let expected = 63;
        assert_eq!(actual, expected);

        let actual = disk.device_grain_size();
        let expected = 1_048_576;
        assert_eq!(actual, expected);

        let actual = disk.device_minimum_io_size();
        let expected = 512;
        assert_eq!(actual, expected);

        let actual = disk.device_optimal_io_size();
        let expected = 512;
        assert_eq!(actual, expected);

        let actual = disk.device_size_in_bytes();
        let expected = 10_485_760;
        assert_eq!(actual, expected);

        let actual = disk.device_size_in_sectors();
        let expected = 20_480;
        assert_eq!(actual, expected);

        let actual = disk.device_bytes_per_logical_sector();
        let expected = 512;
        assert_eq!(actual, expected);

        let actual = disk.device_bytes_per_physical_sector();
        let expected = 512;
        assert_eq!(actual, expected);

        let actual = disk.device_alignment_offset();
        let expected = 0;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    #[ignore]
    fn fdisk_can_override_device_geometry() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let default_cylinders = disk.device_count_cylinders();
        let default_heads = disk.device_count_heads();
        let default_sectors = disk.device_count_sectors();

        let cylinders = 12;
        let heads = 64;
        let sectors = 32;
        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .device_geometry(cylinders, heads, sectors)
            .build()?;

        let actual = disk.has_overriden_device_properties();
        let expected = true;
        assert_eq!(actual, expected);

        // FIXME the tests below fail with the actual != expected. How does a user
        // access the overridden values?
        let actual = disk.device_count_cylinders();
        let expected = cylinders as u64;
        assert_eq!(actual, expected);
        assert_ne!(actual, default_cylinders);

        let actual = disk.device_count_heads();
        let expected = heads as u64;
        assert_eq!(actual, expected);
        assert_ne!(actual, default_heads);

        let actual = disk.device_count_sectors();
        let expected = sectors as u64;
        assert_eq!(actual, expected);
        assert_ne!(actual, default_sectors);

        Ok(())
    }

    #[test]
    #[ignore]
    fn fdisk_can_override_sector_sizes() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let default_phys_size = disk.device_bytes_per_physical_sector();
        let default_logi_size = disk.device_bytes_per_logical_sector();

        let phys_size = 4096; // 4 KiB
        let logi_size = 1024; // 1 KiB
        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .device_sector_sizes(phys_size, logi_size)
            .build()?;

        let actual = disk.has_overriden_device_properties();
        let expected = true;
        assert_eq!(actual, expected);

        // FIXME the tests below fail with the actual != expected. How does a user
        // access the overridden values?
        let actual = disk.device_bytes_per_physical_sector();
        let expected = phys_size as u64;
        assert_eq!(actual, expected);
        assert_ne!(actual, default_phys_size);

        let actual = disk.device_bytes_per_logical_sector();
        let expected = logi_size as u64;
        assert_eq!(actual, expected);
        assert_ne!(actual, default_logi_size);

        Ok(())
    }

    #[test]
    #[ignore]
    fn fdisk_can_override_device_grain_size() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("gpt");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let default_grain_size = disk.device_grain_size();

        let size = 10_485_760; // 10 MiB
        let disk = Fdisk::builder()
            .assign_device(tmp_image.path())
            .device_grain_size(size)
            .build()?;

        let actual = disk.has_overriden_device_properties();
        let expected = true;
        assert_eq!(actual, expected);

        // FIXME the tests below fail with the actual != expected. How does a user
        // access the overridden values?
        let actual = disk.device_grain_size();
        let expected = size;
        assert_eq!(actual, expected);
        assert_ne!(actual, default_grain_size);

        Ok(())
    }

    #[test]
    fn fdisk_can_not_list_empty_spaces_on_blank_device() -> crate::Result<()> {
        let tmp_image = blank_image_file();
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.list_empty_spaces();
        assert!(actual.is_none());

        Ok(())
    }

    #[test]
    fn fdisk_can_list_empty_spaces_on_device_with_partition_table() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("dos_bsd");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.list_empty_spaces();
        assert!(actual.is_some());

        let list = actual.unwrap();
        let actual = list.len();
        let expected = 0;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn fdisk_can_not_list_partitions_on_blank_device() -> crate::Result<()> {
        let tmp_image = blank_image_file();
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.list_partitions();
        assert!(actual.is_none());

        Ok(())
    }

    #[test]
    fn fdisk_can_not_list_partitions_on_unpartitioned_device() -> crate::Result<()> {
        let tmp_image = disk_image_with_fs("ext4");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.list_partitions();
        assert!(actual.is_none());

        Ok(())
    }

    #[test]
    fn fdisk_can_list_partitions_on_partitioned_device() -> crate::Result<()> {
        let tmp_image = disk_image_with_pt("dos_bsd");
        let disk = Fdisk::builder().assign_device(tmp_image.path()).build()?;

        let actual = disk.list_partitions();
        assert!(actual.is_some());

        let list = actual.unwrap();
        let actual = list.len();
        let expected = 2;
        assert_eq!(actual, expected);

        Ok(())
    }
}
