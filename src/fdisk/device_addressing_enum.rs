// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::ffi::CString;
use std::fmt;

// From this library

/// Device addressing unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DeviceAddressing {
    /// Cylinder unit.
    ///
    /// > "A cylinder is a division of data in a disk drive [..]. The concept is concentric, hollow,
    /// > cylindrical slices through the physical disks (platters), collecting the respective circular
    /// > tracks aligned through the stack of platters. The number of cylinders of a disk drive exactly
    /// > equals the number of tracks on a single surface in the drive. It comprises the same track
    /// > number on each platter, spanning all such tracks across each platter surface that is able to
    /// > store data [..]. Cylinders are vertically formed by tracks. In other words, track 12 on platter
    /// > 0 plus track 12 on platter 1 etc. is cylinder 12."
    ///
    /// Source: [Wikipedia - Cylinder-head-sector](https://en.wikipedia.org/wiki/Cylinder-head-sector#Cylinders)
    Cylinder,

    /// Sector unit.
    ///
    /// > "In computer disk storage, a sector is a subdivision of a track on a magnetic disk or optical
    /// > disc. For most disks, each sector stores a fixed amount of user-accessible data, traditionally
    /// > 512 bytes for hard disk drives (HDDs) and 2048 bytes for CD-ROMs and DVD-ROMs. Newer HDDs and
    /// > SSDs use 4096-byte (4 KiB) sectors, which are known as the Advanced Format (AF)."
    ///
    /// Source: [Wikipedia - Disk sector](https://en.wikipedia.org/wiki/Disk_sector)
    Sector,
}

impl DeviceAddressing {
    /// View a `DeviceAddressing` as a UTF-8 `str`.
    fn as_str(&self) -> &str {
        match self {
            Self::Cylinder => "cylinder",
            Self::Sector => "sector",
        }
    }

    /// Converts a `DeviceAddressing` to a [`CString`]
    pub fn to_c_string(&self) -> CString {
        CString::new(self.as_str()).unwrap()
    }
}

impl fmt::Display for DeviceAddressing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
