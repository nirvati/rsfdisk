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
//!     2. [List device partitions](#list-device-partitions)
//!     3. [Partition a device](#partition-a-device)
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
//!
//! ### List device partitions
//!
//! We can use `Fdisk` to list the partitions on a device, mimicking the behaviour of the
//! `fdisk -l /dev/vda` command.
//!
//! ```ignore
//! use terminal_size::{terminal_size, Width};
//! use rsfdisk::core::partition_table::MaxColWidth;
//! use rsfdisk::fdisk::Fdisk;
//! use rsfdisk::fdisk::SizeFormat;
//!
//!
//! fn main() -> rsfdisk::Result<()> {
//!     let disk = Fdisk::builder()
//!         // Read metadata on `/dev/vda`.
//!         .assign_device("/dev/vda")
//!         // Return sizes in a human readable form.
//!         .partition_size_format(SizeFormat::HumanReadable)
//!         .build()?;
//!
//!     let table = disk.partition_table_current().unwrap();
//!     let formats = disk.partition_table_collect_partition_field_formats(table)?;
//!
//!     // Collect column headers.
//!     let headers: Vec<_> = formats
//!         .iter()
//!         .enumerate()
//!         .map(|(i, f)| {
//!             let col_name = f.col_name().unwrap();
//!             format!("({}) {}", i + 1, col_name)
//!         })
//!         .collect();
//!
//!     let mut rows: Vec<String> = vec![];
//!
//!     // Collect and format data about each partition.
//!     let partitions = disk.list_partitions().unwrap();
//!
//!     for partition in partitions.iter() {
//!         let mut columns: Vec<String> = vec![];
//!
//!         for field_format in formats.iter() {
//!             let (Width(w), _) = terminal_size::terminal_size().unwrap();
//!             let mut value = disk.partition_field_to_string(field_format.field(), partition)?;
//!
//!             match field_format.width().unwrap() {
//!                 MaxColWidth::Length(l) => {
//!                     let max_width = l as usize;
//!                     value.truncate(max_width);
//!
//!                     let cell = format!("{:>max_width$} ", value);
//!                     columns.push(cell);
//!                 }
//!                 MaxColWidth::Percentage(p) => {
//!                     let max_width = (w * p / 100) as usize;
//!                     value.truncate(max_width);
//!
//!                     let cell = format!("{:max_width$} ", value);
//!                     columns.push(cell);
//!                 }
//!             }
//!         }
//!
//!         rows.push(columns.join(" "));
//!     }
//!
//!     println!("Columns: {}\n", headers.join(" "));
//!     println!("{}", rows.join("\n"));
//!
//!     // Example output
//!     //
//!     // Columns: (1) Device (2) Boot (3) Start (4) End (5) Sectors (6) Size (7) Id (8) Type
//!     //
//!     // /dev/vda1  *    32   7679   7648   3.7M  83  Linux
//!     // /dev/vda2     7680  16383   8704   4.3M  a5  FreeBSD
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Partition a device
//!
//! In this example we will divide a device into three partitions:
//! - a 16 GiB `root` partition to keep system files,
//! - and two 64 GiB data partitions.
//!
//! We let [`Fdisk`] take care of positioning and numbering the resulting logical disks on `/dev/vda`.
//!
//! ```ignore
//! use rsfdisk::fdisk::Fdisk;
//! use rsfdisk::core::partition_table::PartitionTableKind;
//! use rsfdisk::core::partition::Guid;
//! use rsfdisk::core::partition::Partition;
//! use rsfdisk::core::partition::PartitionKind;
//! use rsfdisk::core::partition::PartitionList;
//!
//! fn main() -> rsfdisk::Result<()> {
//!     let mut disk = Fdisk::builder()
//!         // Operate on `/dev/vda`.
//!         .assign_device("/dev/vda")
//!         // Allow Fdisk to persist changes to disk.
//!         .enable_read_write()
//!         // Remove all existing partition tables, file systems, and RAID signatures on the
//!         // assigned device before writing a new partition table.
//!         .wipe_device_metadata()
//!         .build()?;
//!
//!     // Create a `GPT` partition table.
//!     disk.partition_table_create(PartitionTableKind::GPT)?;
//!
//!     // Configure a 16 GiB System partition
//!     let partition_type = PartitionKind::builder()
//!        // Set the partition type identifier for a GUID/GPT partition table.
//!        .guid(Guid::LinuxRootx86_64)
//!        .build()?;
//!
//!     let root = Partition::builder()
//!        .partition_type(partition_type)
//!        .name("System")
//!        //Assuming 512 bytes per sector, 33,554,432 sectors <=> 16 GiB.
//!        .size_in_sectors(33_554_432)
//!        .build()?;
//!
//!     // Create the root partition.
//!     let _ = disk.partition_add(root)?;
//!
//!     // Configure two 64 GiB data partitions.
//!     let mut data_partitions = PartitionList::new()?;
//!
//!     // Assuming 512 bytes per sector, 68,719,476,736 sectors <=> 64 GiB.
//!     let size = 68_719_476_736;
//!
//!     for i in 0..2 {
//!         let partition_type = PartitionKind::builder()
//!            .guid(Guid::LinuxData)
//!            .build()?;
//!
//!         let name = format!("Data Part {}", i + 1);
//!
//!         let partition = Partition::builder()
//!            .partition_type(partition_type)
//!            .name(name)
//!            .size_in_sectors(size)
//!            .build()?;
//!
//!         data_partitions.push(partition)?;
//!     }
//!
//!     // Create the data partitions.
//!     disk.partitions_append(data_partitions)?;
//!
//!     // Write the new partition table on `/dev/vda`.
//!     disk.partition_table_write_to_disk()?;
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
pub use fdisk_dos_ext_trait::FdiskDOSExt;
pub use fdisk_error_enum::FdiskError;
pub use fdisk_gpt_ext_trait::FdiskGPTExt;
pub use fdisk_struct::Fdisk;
pub use fdisk_sun_ext_trait::FdiskSUNExt;
pub(crate) use gc_item_enum::GcItem;
pub(crate) use lba_align_enum::LBAAlign;
pub use partition_table_iter_mut_struct::PartitionTableIterMut;
pub use partition_table_iter_struct::PartitionTableIter;
pub use size_format_enum::SizeFormat;

mod device_addressing_enum;
mod fdisk_builder_error_enum;
mod fdisk_builder_struct;
mod fdisk_dos_ext_trait;
mod fdisk_error_enum;
mod fdisk_gpt_ext_trait;
mod fdisk_struct;
mod fdisk_sun_ext_trait;
mod gc_item_enum;
mod lba_align_enum;
mod partition_table_iter_mut_struct;
mod partition_table_iter_struct;
mod size_format_enum;
