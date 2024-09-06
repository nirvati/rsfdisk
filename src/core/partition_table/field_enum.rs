// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::{IntoPrimitive, TryFromPrimitive};

// From standard library

// From this library

/// Fields of a partition entry in a partition table.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum Field {
    /// Unspecified item.
    NotSpecified = libfdisk::fdisk_fieldtype_FDISK_FIELD_NONE,

    /// Partition device name.
    Device = libfdisk::fdisk_fieldtype_FDISK_FIELD_DEVICE,

    /// Indicator of bootable legacy partitions.
    BootIndicator = libfdisk::fdisk_fieldtype_FDISK_FIELD_BOOT,

    /// Start of partition in Cylinder-Head-Sector (CHS) address format (MBR).
    ChsStartingSector = libfdisk::fdisk_fieldtype_FDISK_FIELD_SADDR,

    /// Type of partition.
    TypeId = libfdisk::fdisk_fieldtype_FDISK_FIELD_TYPEID,

    /// End of partition in Cylinder-Head-Sector (CHS) address format (MBR).
    ChsEndingSector = libfdisk::fdisk_fieldtype_FDISK_FIELD_EADDR,

    /// Start of partition in Logical Block Addressing (LBA) address format.
    StartingLba = libfdisk::fdisk_fieldtype_FDISK_FIELD_START,

    /// Size of the partition in LBA units of logical blocks.
    Size = libfdisk::fdisk_fieldtype_FDISK_FIELD_SIZE,

    /// End of partition in Logical Block Addressing (LBA) address format.
    EndingLba = libfdisk::fdisk_fieldtype_FDISK_FIELD_END,

    /// Attribute bits, all bits reserved by UEFI (GPT).
    AttributeBits = libfdisk::fdisk_fieldtype_FDISK_FIELD_ATTR,

    /// Null-terminated string containing a human-readable name of the partition (GPT).
    Name = libfdisk::fdisk_fieldtype_FDISK_FIELD_NAME,

    /// Unique ID that defines the purpose and type of this Partition (GPT).
    Type = libfdisk::fdisk_fieldtype_FDISK_FIELD_TYPE,

    /// GUID that is unique for every partition entry (GPT).
    Uuid = libfdisk::fdisk_fieldtype_FDISK_FIELD_UUID,

    /// Null-terminated string containing a human-readable name for a formatted partition.
    FileSystemLabel = libfdisk::fdisk_fieldtype_FDISK_FIELD_FSLABEL,

    /// File system type.
    FileSystemType = libfdisk::fdisk_fieldtype_FDISK_FIELD_FSTYPE,

    /// Unique file system ID.
    FileSystemUuid = libfdisk::fdisk_fieldtype_FDISK_FIELD_FSUUID,

    /// For 4.2BSD file systems only, the size of a file system block, in bytes.
    BlockSize = libfdisk::fdisk_fieldtype_FDISK_FIELD_BSIZE,

    /// For 4.2BSD file systems, the number of cylinders in a cylinder group.
    CylindersPerGroup = libfdisk::fdisk_fieldtype_FDISK_FIELD_CPG,

    /// For 4.2BSD file systems only, the fragment size of the file system in bytes.
    FragmentSize = libfdisk::fdisk_fieldtype_FDISK_FIELD_FSIZE,

    /// Number of cylinders (deprecated).
    NumberOfCylinders = libfdisk::fdisk_fieldtype_FDISK_FIELD_CYLINDERS,

    /// Number of sectors.
    NumberOfSectors = libfdisk::fdisk_fieldtype_FDISK_FIELD_SECTORS,
    // FieldCounter = libfdisk::fdisk_fieldtype_FDISK_NFIELDS,
}
