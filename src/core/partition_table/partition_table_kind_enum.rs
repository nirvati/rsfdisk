// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::{IntoPrimitive, TryFromPrimitive};

// From standard library
use std::ffi::CString;
use std::fmt;

// From this library

/// Supported types of partition tables.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum PartitionTableKind {
    BSD = libfdisk::fdisk_labeltype_FDISK_DISKLABEL_BSD,
    DOS = libfdisk::fdisk_labeltype_FDISK_DISKLABEL_DOS,
    GPT = libfdisk::fdisk_labeltype_FDISK_DISKLABEL_GPT,
    SGI = libfdisk::fdisk_labeltype_FDISK_DISKLABEL_SGI,
    SUN = libfdisk::fdisk_labeltype_FDISK_DISKLABEL_SUN,
}

impl PartitionTableKind {
    /// View this `PartitionTableKind` as a UTF-8 `str.`
    pub fn as_str(&self) -> &str {
        match self {
            Self::BSD => "bsd",
            Self::DOS => "dos",
            Self::GPT => "gpt",
            Self::SGI => "sgi",
            Self::SUN => "sun",
        }
    }

    pub fn to_c_string(&self) -> CString {
        CString::new(self.as_str()).unwrap()
    }
}

impl fmt::Display for PartitionTableKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
