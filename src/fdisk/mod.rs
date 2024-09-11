// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `rsfdisk`'s main module.
//!
//! ----
//! # Table of Contents
//! 1. [Description](#description)
//! 2. [Overview](#overview)
//! 3. [Usage](#usage)
//! 4. [Examples](#examples)
//!     1. [Read device topology](#read-device-topology)
//!
//! ## Description
//!
//! The `fdisk` module holds the main objects a user needs to read, edit, create partition
//! tables and/or partitions on a device. The most important being the [`Fdisk`] struct which
//! centralizes all module functionalities.
//!
//! ## Overview
//!
//! [`Fdisk`] offers a variety of tools to operate on a device, both programmatically or
//! interactively to gather the data necessary for editing or creating partition tables.
//!
//! When a device is assigned, [`Fdisk`] extracts topology, geometry, and partition table metadata.
//! Users can override some values resulting from this initial device scan before proceeding to
//! read, edit, or create partition tables on the disk.
//!
//! ## Usage
//!
//! To manipulate devices, `rsfdisk` provides the [`FdiskBuilder`] struct, to configure and create a
//! new instance of [`Fdisk`]. Through [`FdiskBuilder`], a user can specify:
//! - the device to operate on, which can either be a physical block device or an image file,
//! - whether to enable the interactive mode for device partitioning,
//! - which/how data should be displayed on the console in interactive mode,
//! - whether to erase old partition tables present on the device,
//! - etc.
//!
//! ## Examples
//!
//! ### Read device topology
//!
//! We can use `Fdisk` to get the characteristics of a device, mimicking the behaviour of the
//! `fdisk -l /dev/vda` command.
//!
//! ```ignore
//! use rsfdisk::fdisk::Fdisk;
//!
//! fn main() -> rsfdisk::Result<()> {
//!     let disk = Fdisk::builder()
//!         // Read metadata on `/dev/vda`.
//!         .assign_device("/dev/vda")
//!         .build()?;
//!
//!     let device_name = disk.device_name().unwrap();
//!     let size_in_bytes = disk.device_size_in_bytes();
//!     let size = size_in_bytes >> 30; // convert to GiB
//!     let size_in_sectors = disk.device_size_in_sectors();
//!     let bytes_per_logical_sector = disk.device_bytes_per_logical_sector();
//!     let bytes_per_physical_sector = disk.device_bytes_per_physical_sector();
//!     let minimum_io_size = disk.device_minimum_io_size();
//!     let optimal_io_size = disk.device_optimal_io_size();
//!
//!     println!("Disk {}: {size} GiB, {size_in_bytes} bytes, {size_in_sectors} sectors", device_name.display());
//!     println!("Sector size (logical/physical): {bytes_per_logical_sector} bytes / {bytes_per_physical_sector} bytes");
//!     println!("I/O size (minimum/optimal): {minimum_io_size} bytes / {optimal_io_size} bytes");
//!
//!     // Example output
//!     //
//!     // Disk /dev/vda: 8 GiB, 8589934592 bytes, 16777216 sectors
//!     // Sector size (logical/physical): 512 bytes / 512 bytes
//!     // I/O size (minimum/optimal): 512 bytes / 512 bytes
//!
//!     Ok(())
//! }
//! ```

// From dependency library

// From standard library

// From this library

pub use device_addressing_enum::DeviceAddressing;
pub use fdisk_builder_error_enum::FdiskBuilderError;
pub(crate) use fdisk_builder_struct::CtxBuilder;
pub use fdisk_builder_struct::FdiskBuilder;
pub use fdisk_error_enum::FdiskError;
pub use fdisk_struct::Fdisk;
pub(crate) use gc_item_enum::GcItem;
#[allow(unused_imports)]
pub(crate) use lba_align_enum::LBAAlign;
pub use partition_table_iter_mut_struct::PartitionTableIterMut;
pub use partition_table_iter_struct::PartitionTableIter;
pub use size_format_enum::SizeFormat;

mod device_addressing_enum;
mod fdisk_builder_error_enum;
mod fdisk_builder_struct;
mod fdisk_error_enum;
mod fdisk_struct;
mod gc_item_enum;
mod lba_align_enum;
mod partition_table_iter_mut_struct;
mod partition_table_iter_struct;
mod size_format_enum;
