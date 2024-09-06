// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::{IntoPrimitive, TryFromPrimitive};

// From standard library

// From this library

// FIXME discriminant values assigned more than once
// I suppose *_LABELITEM_* should be monotonically increasing judging by the definition of
// __FDISK_NLABELITEMS     util-linux/libfdisk/src/libfdisk.h.in:l423 which is used to initialize
// SUN_LABELITEM_LABELID (l647), BSD_LABELITEM_TYPE (l682), SGI_LABELITEM_PCYLCOUNT (l714),
// GPT_LABELITEM_FIRSTLBA (l791)
// why does it not?
// Workaround to error message "discriminant values assigned more than once".
const BSD_OFFSET: u32 = 16;
const GPT_OFFSET: u32 = 64;
const SGI_OFFSET: u32 = 256;
const SUN_OFFSET: u32 = 1024;

/// Fields in a Partition Table Header.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum HeaderEntry {
    //---- BEGIN BSD
    // Description from https://man.openbsd.org/disktab.5
    /// Type of controller (e.g., SMD, ESDI, floppy).
    BsdType = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_TYPE + BSD_OFFSET,

    /// Number of tracks per cylinder.
    BsdTracksPerCylinder = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_NTRACKS + BSD_OFFSET,

    /// Number of sectors per cylinder.
    BsdSectorsPerCylinder = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_SECPERCYL + BSD_OFFSET,

    /// Total number of cylinders on the disk.
    BsdCylindersTotal = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_CYLINDERS + BSD_OFFSET,

    /// Sector size in bytes.
    BsdSectorSize = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_SECSIZE + BSD_OFFSET,

    BsdCylinderSkew = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_CYLINDERSKEW + BSD_OFFSET,
    BsdDisk = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_DISK + BSD_OFFSET,
    BsdFlags = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_FLAGS + BSD_OFFSET,
    BsdHeadSwitch = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_HEADSWITCH + BSD_OFFSET,
    BsdInterlave = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_INTERLEAVE + BSD_OFFSET,
    BsdPackName = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_PACKNAME + BSD_OFFSET,
    BsdRpm = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_RPM + BSD_OFFSET,
    BsdTrackSkew = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_TRACKSKEW + BSD_OFFSET,
    BsdTrackToTrackSeek = libfdisk::fdisk_labelitem_bsd_BSD_LABELITEM_TRKSEEK + BSD_OFFSET,
    //---- END BSD
    GenericId = libfdisk::fdisk_labelitem_gen_FDISK_LABELITEM_ID,

    //---- BEGIN GPT
    /// LBA address of the alternate GPT Header.
    GptAlternateLba = libfdisk::fdisk_labelitem_gpt_GPT_LABELITEM_ALTLBA + GPT_OFFSET,

    /// The first usable logical block that may be used by a partition described by a GUID
    /// Partition Entry.
    GptFirstUsableLba = libfdisk::fdisk_labelitem_gpt_GPT_LABELITEM_FIRSTLBA + GPT_OFFSET,

    /// The last usable logical block that may be used by a partition described by a GUID Partition
    /// Entry.
    GptLastUsableLba = libfdisk::fdisk_labelitem_gpt_GPT_LABELITEM_LASTLBA + GPT_OFFSET,

    /// GUID that can be used to uniquely identify the disk.
    GptDiskGuid = libfdisk::fdisk_labelitem_gpt_GPT_LABELITEM_ID + GPT_OFFSET,

    /// The starting LBA of the GUID Partition Entry array.
    GptPartitionEntryFirstLba = libfdisk::fdisk_labelitem_gpt_GPT_LABELITEM_ENTRIESLBA + GPT_OFFSET,

    #[cfg(v2_39)]
    /// The ending LBA of the GUID Partition Entry array.
    GptPartitionEntryLastLba =
        libfdisk::fdisk_labelitem_gpt_GPT_LABELITEM_ENTRIESLASTLBA + GPT_OFFSET,

    /// The maximum number of Partition Entries in the GUID Partition Entry array.
    GptMaxNumberOfPartitionEntries =
        libfdisk::fdisk_labelitem_gpt_GPT_LABELITEM_ENTRIESALLOC + GPT_OFFSET,
    //---- END GPT

    //---- BEGIN SGI
    SgiBootfile = libfdisk::fdisk_labelitem_sgi_SGI_LABELITEM_BOOTFILE + SGI_OFFSET,
    SgiInterleave = libfdisk::fdisk_labelitem_sgi_SGI_LABELITEM_ILFACT + SGI_OFFSET,
    SgiPhysicalCylindersCount = libfdisk::fdisk_labelitem_sgi_SGI_LABELITEM_PCYLCOUNT + SGI_OFFSET,
    SgiSpareSectorsPerCylinder = libfdisk::fdisk_labelitem_sgi_SGI_LABELITEM_SPARECYL + SGI_OFFSET,
    //---- END SGI

    //---- BEGIN SUN
    SunAlternateCylinders = libfdisk::fdisk_labelitem_sun_SUN_LABELITEM_ACYL + SUN_OFFSET,
    SunExtraSectorsPerCylinder = libfdisk::fdisk_labelitem_sun_SUN_LABELITEM_APC + SUN_OFFSET,
    SunInterleave = libfdisk::fdisk_labelitem_sun_SUN_LABELITEM_INTRLV + SUN_OFFSET,
    SunPartitionTableType = libfdisk::fdisk_labelitem_sun_SUN_LABELITEM_LABELID + SUN_OFFSET,
    SunPhysicalCylinders = libfdisk::fdisk_labelitem_sun_SUN_LABELITEM_PCYL + SUN_OFFSET,
    SunRpm = libfdisk::fdisk_labelitem_sun_SUN_LABELITEM_RPM + SUN_OFFSET,
    SunVolumeId = libfdisk::fdisk_labelitem_sun_SUN_LABELITEM_VTOCID + SUN_OFFSET,
    //---- END SUN
}

impl HeaderEntry {
    /// Converts each variant to its corresponding enum value in `libfdisk`.
    pub fn to_original_u32(&self) -> u32 {
        let value: u32 = (*self).into();
        let offset = match self {
            Self::GenericId => 0,
            Self::BsdCylindersTotal
            | Self::BsdCylinderSkew
            | Self::BsdDisk
            | Self::BsdFlags
            | Self::BsdHeadSwitch
            | Self::BsdInterlave
            | Self::BsdTracksPerCylinder
            | Self::BsdPackName
            | Self::BsdRpm
            | Self::BsdSectorsPerCylinder
            | Self::BsdSectorSize
            | Self::BsdTrackSkew
            | Self::BsdTrackToTrackSeek
            | Self::BsdType => BSD_OFFSET,
            Self::GptAlternateLba
            | Self::GptMaxNumberOfPartitionEntries
            | Self::GptPartitionEntryFirstLba
            | Self::GptFirstUsableLba
            | Self::GptDiskGuid
            | Self::GptLastUsableLba => GPT_OFFSET,
            #[cfg(v2_39)]
            Self::GptPartitionEntryLastLba => GPT_OFFSET,
            Self::SgiBootfile
            | Self::SgiInterleave
            | Self::SgiPhysicalCylindersCount
            | Self::SgiSpareSectorsPerCylinder => SGI_OFFSET,
            Self::SunAlternateCylinders
            | Self::SunExtraSectorsPerCylinder
            | Self::SunInterleave
            | Self::SunPartitionTableType
            | Self::SunPhysicalCylinders
            | Self::SunRpm
            | Self::SunVolumeId => SUN_OFFSET,
        };

        value - offset
    }
}
