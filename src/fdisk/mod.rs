// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `rsfdisk`'s main module.

// From dependency library

// From standard library

// From this library

pub use device_addressing_enum::DeviceAddressing;
pub use fdisk_error_enum::FdiskError;
pub use fdisk_struct::Fdisk;
pub(crate) use gc_item_enum::GcItem;
pub use size_format_enum::SizeFormat;

mod device_addressing_enum;
mod fdisk_error_enum;
mod fdisk_struct;
mod gc_item_enum;
mod size_format_enum;
