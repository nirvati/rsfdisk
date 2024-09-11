// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::{IntoPrimitive, TryFromPrimitive};

// From standard library

// From this library

/// Display format of partition sizes.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum SizeFormat {
    /// Display sizes in bytes.
    Bytes = libfdisk::FDISK_SIZEUNIT_BYTES,

    /// Display sizes in KB, MB, GB.
    HumanReadable = libfdisk::FDISK_SIZEUNIT_HUMAN,
}
