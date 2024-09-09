// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use enum_iterator::Sequence;
use num_enum::{IntoPrimitive, TryFromPrimitive};

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Supported `MBR` partitions.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Sequence, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
#[non_exhaustive]
pub enum Code {
    /// Empty partition entry.
    EmptyPartition = 0x00,

    /// XENIX root.
    FAT12 = 0x01,

    /// XENIX root.
    XenixRoot = 0x02,

    /// XENIX usr.
    XenixUser = 0x03,

    /// FAT16 with less than 65,536 sectors (32 MB).
    FAT16 = 0x04,

    /// Extended partition with CHS addressing.
    ExtendedPartition = 0x05,

    /// FAT16B with 65,536 or more sectors.
    FAT16B = 0x06,

    /// HPFS / NTFS / exFAT.
    HPFSNTFSExfat = 0x07,

    /// AIX boot/split.
    AIX = 0x08,

    /// AIX data/boot.
    AIXBootable = 0x09,

    /// OS/2 Boot Manager
    OS2BootManager = 0x0a,

    /// FAT32 with CHS addressing.
    W95FAT32 = 0x0b,

    /// FAT32 with LBA.
    W95FAT32LBA = 0x0c,

    /// FAT16B with LBA.
    W95FAT16LBA = 0x0e,

    /// Extended partition with LBA.
    W95ExtendedLBA = 0x0f,

    /// OPUS.
    OPUS = 0x10,

    /// Hidden FAT12.
    HiddenFAT12 = 0x11,

    /// Diagnostics and firmware partition (bootable FAT).
    CompaqDiagnostics = 0x12,

    /// Hidden FAT16.
    HiddenFAT16 = 0x14,

    /// Hidden FAT16B.
    HiddenFAT16B = 0x16,

    /// Hidden HPFS / NTFS / exFAT.
    HiddenHPFSNTFSExFat = 0x17,

    /// AST SmartSleep partition.
    ASTSmartSleep = 0x18,

    /// Hidden FAT32 with CHS addressing.
    HiddenW95FAT32 = 0x1b,

    /// Hidden FAT32 with LBA.
    HiddenW95FAT32LBA = 0x1c,

    /// Hidden FAT16B with LBA.
    HiddenW95FAT16LBA = 0x1e,

    /// NEC MS-DOS 3.30 Logical sectored FAT12 or FAT16.
    NecDOS = 0x24,

    /// Hidden NTFS rescue partition.
    HiddenNTFSRescue = 0x27,

    /// Plan 9 edition 3 partition.
    Plan9 = 0x39,

    /// PartitionMagic recovery partition.
    PartitionMagic = 0x3c,

    /// Venix 80286.
    Venix80286 = 0x40,

    /// PPC PReP (Power PC Reference Platform) Boot.
    PPCPrepBoot = 0x41,

    /// Secure File system (SFS).
    Sfs = 0x42,

    /// Primary QNX POSIX volume on disk .
    QNX4Primary = 0x4d,

    /// Secondary QNX POSIX volume on disk.
    QNX4Secondary = 0x4e,

    /// Tertiary QNX POSIX volume on disk.
    QNX4Tertiary = 0x4f,

    /// OnTrack Disk Manager 4 read-only partition.
    OnTrackDM = 0x50,

    /// OnTrack Disk Manager 4-6 read-write partition (Aux 1).
    OnTrackDM6Aux1 = 0x51,

    /// CP/M-80.
    CPM80 = 0x52,

    /// Disk Manager 6 Auxiliary 3 (WO).
    OnTrackDM6Aux3 = 0x53,

    /// Disk Manager 6 Dynamic Drive Overlay (DDO).
    OnTrackDM6Ddo = 0x54,

    /// EZ-Drive.
    EZDrive = 0x55,

    /// Golden Bow VFeature Partitioned Volume.
    GoldenBow = 0x56,

    /// Priam EDisk Partitioned Volume.
    PriamEDisk = 0x5c,

    /// SpeedStor Hidden FAT12.
    SpeedStor = 0x61,

    /// Unix System V (SCO, ISC Unix, UnixWare, ...), Mach, GNU Hurd.
    GNUHurdSystemV = 0x63,

    /// Novell Netware 286, 2.xx
    NovellNetware286 = 0x64,

    /// Novell Netware 386, 3.xx or 4.xx
    NovellNetware386 = 0x65,

    /// DiskSecure multiboot.
    DiskSecureMultiBoot = 0x70,

    /// PC/IX.
    PCIX = 0x75,

    /// Minix 1.1-1.4a MINIX file system (old).
    OldMinix = 0x80,

    /// Minix 1.4b+ MINIX file system.
    MinixOldLinux = 0x81,

    /// Linux SWAP space.
    LinuxSwap = 0x82,

    /// Native Linux file system.
    Linux = 0x83,

    /// OS/2 hidden C: drive.
    OS2HiddenCDrive = 0x84,

    /// Linux extended partition.
    LinuxExtended = 0x85,

    /// Fault-tolerant FAT16B mirrored volume set.
    FAT16VolumeSet = 0x86,

    /// Fault-tolerant HPFS/NTFS mirrored volume set.
    NTFSVolumeSet = 0x87,

    /// Linux plain text partition table .
    LinuxPlaintext = 0x88,

    /// Linux Logical Volume Manager partition.
    LinuxLVM = 0x8e,

    /// Amoeba native file system.
    Amoeba = 0x93,

    /// Amoeba bad block table.
    AmoebaBadBlockTable = 0x94,

    /// BSD/OS 3.0+, BSDI.
    BSDOs = 0x9f,

    /// IBM Thinkpad Laptop hibernation partition.
    IBMThinkpad = 0xa0,

    /// FreeBSD.
    FreeBSD = 0xa5,

    /// OpenBSD.
    OpenBSD = 0xa6,

    /// NeXTSTEP.
    NextStep = 0xa7,

    /// Apple Darwin, Mac OS X UFS.
    DarwinUFS = 0xa8,

    /// NetBSD slice.
    NetBSD = 0xa9,

    /// Apple Darwin, Mac OS X boot.
    DarwinBoot = 0xab,

    /// HFS and HFS+
    HFSHFSPlus = 0xaf,

    /// BSDI native file system.
    BSDIFs = 0xb7,

    /// BSDI native swap.
    BSDISwap = 0xb8,

    /// PTS BootWizard 4 / OS Selector 5 for hidden partitions.
    BootWizardHidden = 0xbb,

    /// Acronis backup partition (Acronis Secure Zone).
    AcronisFAT32LBA = 0xbc,

    /// Solaris 8 boot partition.
    SolarisBoot = 0xbe,

    /// New Solaris x86 partition.
    Solaris = 0xbf,

    /// DR DOS 6.0+ Secured FAT12.
    DRDOSSecuredFAT12 = 0xc1,

    /// DR DOS 6.0+ Secured FAT16.
    DRDOSSecuredFAT16 = 0xc4,

    /// DR DOS 6.0+ Secured FAT16B.
    DRDOSSecuredFAT16B = 0xc6,

    /// Syrinx boot.
    Syrinx = 0xc7,

    /// Non-file system data.
    NonFsData = 0xda,

    /// Digital Research CP/M, Concurrent CP/M, Concurrent DOS.
    CPMCtOs = 0xdb,

    /// Dell PowerEdge Server utilities (FAT16).
    DellUtilityFAT16 = 0xde,

    /// BootIt EMBRM.
    BootIt = 0xdf,

    /// DOS access or SpeedStor 12-bit FAT extended partition.
    DOSAccess = 0xe1,

    /// SpeedStor Read-only FAT12.
    DOSRO = 0xe3,

    /// SpeedStor 16-bit FAT extended partition < 1024 cylinders.
    SpeedStorFAT16 = 0xe4,

    /// Freedesktop boot.
    FreedesktopBoot = 0xea,

    /// BeOS, Haiku BFS.
    BeOSBFS = 0xeb,

    /// GPT protective MBR (indication that this legacy MBR is followed by an EFI header).
    GPTProtectiveMBR = 0xee,

    /// EFI system partition. Can be a FAT12, FAT16, FAT32 (or other) file system.
    EfiSystem = 0xef,

    /// PA-RISC Linux boot loader.
    PARISCLinux = 0xf0,

    /// Storage Dimensions SpeedStor.
    SDSpeedstor = 0xf1,

    /// SpeedStor large partition.
    SpeedStorFAT16B = 0xf4,

    /// DOS 3.3+ secondary partition.
    DOSSecondary = 0xf2,

    /// Arm EBBR 1.0 Protective partition for the area containing system firmware.
    EBBRProtective = 0xf8,

    /// VMware ESX VMware VMFS file system partition.
    VMWareVMFS = 0xfb,

    /// VMware ESX VMware swap / VMKCORE kernel dump partition.
    VMWareVMKCORE = 0xfc,

    /// Linux RAID superblock with auto-detect.
    LinuxRaidAuto = 0xfd,

    /// LANstep.
    LanStep = 0xfe,

    ///  Xenix Bad Block Table.
    XenixBadBlockTable = 0xff,
}

impl Code {
    /// View this `Code` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::EmptyPartition => "0x00",
            Self::FAT12 => "0x01",
            Self::XenixRoot => "0x02",
            Self::XenixUser => "0x03",
            Self::FAT16 => "0x04",
            Self::ExtendedPartition => "0x05",
            Self::FAT16B => "0x06",
            Self::HPFSNTFSExfat => "0x07",
            Self::AIX => "0x08",
            Self::AIXBootable => "0x09",
            Self::OS2BootManager => "0x0a",
            Self::W95FAT32 => "0x0b",
            Self::W95FAT32LBA => "0x0c",
            Self::W95FAT16LBA => "0x0e",
            Self::W95ExtendedLBA => "0x0f",
            Self::OPUS => "0x10",
            Self::HiddenFAT12 => "0x11",
            Self::CompaqDiagnostics => "0x12",
            Self::HiddenFAT16 => "0x14",
            Self::HiddenFAT16B => "0x16",
            Self::HiddenHPFSNTFSExFat => "0x17",
            Self::ASTSmartSleep => "0x18",
            Self::HiddenW95FAT32 => "0x1b",
            Self::HiddenW95FAT32LBA => "0x1c",
            Self::HiddenW95FAT16LBA => "0x1e",
            Self::NecDOS => "0x24",
            Self::HiddenNTFSRescue => "0x27",
            Self::Plan9 => "0x39",
            Self::PartitionMagic => "0x3c",
            Self::Venix80286 => "0x40",
            Self::PPCPrepBoot => "0x41",
            Self::Sfs => "0x42",
            Self::QNX4Primary => "0x4d",
            Self::QNX4Secondary => "0x4e",
            Self::QNX4Tertiary => "0x4f",
            Self::OnTrackDM => "0x50",
            Self::OnTrackDM6Aux1 => "0x51",
            Self::CPM80 => "0x52",
            Self::OnTrackDM6Aux3 => "0x53",
            Self::OnTrackDM6Ddo => "0x54",
            Self::EZDrive => "0x55",
            Self::GoldenBow => "0x56",
            Self::PriamEDisk => "0x5c",
            Self::SpeedStor => "0x61",
            Self::GNUHurdSystemV => "0x63",
            Self::NovellNetware286 => "0x64",
            Self::NovellNetware386 => "0x65",
            Self::DiskSecureMultiBoot => "0x70",
            Self::PCIX => "0x75",
            Self::OldMinix => "0x80",
            Self::MinixOldLinux => "0x81",
            Self::LinuxSwap => "0x82",
            Self::Linux => "0x83",
            Self::OS2HiddenCDrive => "0x84",
            Self::LinuxExtended => "0x85",
            Self::FAT16VolumeSet => "0x86",
            Self::NTFSVolumeSet => "0x87",
            Self::LinuxPlaintext => "0x88",
            Self::LinuxLVM => "0x8e",
            Self::Amoeba => "0x93",
            Self::AmoebaBadBlockTable => "0x94",
            Self::BSDOs => "0x9f",
            Self::IBMThinkpad => "0xa0",
            Self::FreeBSD => "0xa5",
            Self::OpenBSD => "0xa6",
            Self::NextStep => "0xa7",
            Self::DarwinUFS => "0xa8",
            Self::NetBSD => "0xa9",
            Self::DarwinBoot => "0xab",
            Self::HFSHFSPlus => "0xaf",
            Self::BSDIFs => "0xb7",
            Self::BSDISwap => "0xb8",
            Self::BootWizardHidden => "0xbb",
            Self::AcronisFAT32LBA => "0xbc",
            Self::SolarisBoot => "0xbe",
            Self::Solaris => "0xbf",
            Self::DRDOSSecuredFAT12 => "0xc1",
            Self::DRDOSSecuredFAT16 => "0xc4",
            Self::DRDOSSecuredFAT16B => "0xc6",
            Self::Syrinx => "0xc7",
            Self::NonFsData => "0xda",
            Self::CPMCtOs => "0xdb",
            Self::DellUtilityFAT16 => "0xde",
            Self::BootIt => "0xdf",
            Self::DOSAccess => "0xe1",
            Self::DOSRO => "0xe3",
            Self::SpeedStorFAT16 => "0xe4",
            Self::FreedesktopBoot => "0xea",
            Self::BeOSBFS => "0xeb",
            Self::GPTProtectiveMBR => "0xee",
            Self::EfiSystem => "0xef",
            Self::PARISCLinux => "0xf0",
            Self::SDSpeedstor => "0xf1",
            Self::SpeedStorFAT16B => "0xf4",
            Self::DOSSecondary => "0xf2",
            Self::EBBRProtective => "0xf8",
            Self::VMWareVMFS => "0xfb",
            Self::VMWareVMKCORE => "0xfc",
            Self::LinuxRaidAuto => "0xfd",
            Self::LanStep => "0xfe",
            Self::XenixBadBlockTable => "0xff",
        }
    }

    /// Converts this `Code` to a `u32`.
    pub fn to_u32(&self) -> u32 {
        *self as u8 as u32
    }
}

impl AsRef<Code> for Code {
    #[inline]
    fn as_ref(&self) -> &Code {
        self
    }
}

impl AsRef<str> for Code {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for Code {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::Code(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| Self::from_str(s).map_err(|e| ConversionError::Code(e.to_string())))
    }
}

impl TryFrom<Vec<u8>> for Code {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for Code {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove opening opening/closing quotes/double-quotes if present
        let err_missing_dquote = format!("missing closing double-quote in: {}", s);
        let err_missing_quote = format!("missing closing quote in: {}", s);

        let trimmed = s.trim();
        let stripped = if trimmed.starts_with('"') {
            trimmed
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or(ParserError::Code(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::Code(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        // Remove hex string prefix and convert to `Code`.
        stripped
            .trim()
            .strip_prefix("0x")
            .ok_or(ParserError::Code(format!("missing '0x' prefix in: {}", s)))
            .and_then(|h| {
                u8::from_str_radix(h, 16).map_err(|e| {
                    let err_msg = format!("invalid hexadecimal string: {} {:?}", s, e);

                    ParserError::Code(err_msg)
                })
            })
            .and_then(|n| {
                Self::try_from(n).map_err(|_| {
                    let err_msg = format!("unsupported OS type: {}", s);

                    ParserError::Code(err_msg)
                })
            })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn code_can_not_parse_a_code_string_with_an_unclosed_double_quote() {
        let _: Code = r#""0x82"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn code_can_not_parse_a_code_string_with_an_unclosed_quote() {
        let _: Code = "'0x82".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing '0x' prefix")]
    fn code_can_not_parse_an_empty_string() {
        let _: Code = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing '0x' prefix")]
    fn code_can_not_parse_a_code_missing_its_0x_prefix() {
        let _: Code = "82".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid hexadecimal string")]
    fn code_can_not_parse_a_code_string_with_an_invalid_hexadecimal() {
        let _: Code = "0xDUMMY".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid hexadecimal string")]
    fn code_can_not_parse_a_code_string_with_a_too_large_hexadecimal() {
        let _: Code = "0xffffff".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn code_can_not_convert_invalid_bytes_into_a_code() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = Code::try_from(bytes).unwrap();
    }

    #[test]
    fn code_can_convert_valid_bytes_into_a_code() -> crate::Result<()> {
        let bytes: Vec<u8> = b"0x83".to_vec();
        let actual = Code::try_from(bytes)?;
        let expected = Code::Linux;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn code_can_parse_a_valid_device_code() -> crate::Result<()> {
        let code_str = "0x00";
        let actual: Code = code_str.parse()?;
        let expected = Code::EmptyPartition;
        assert_eq!(actual, expected);

        let code_str = "0x01";
        let actual: Code = code_str.parse()?;
        let expected = Code::FAT12;
        assert_eq!(actual, expected);

        let code_str = "0x02";
        let actual: Code = code_str.parse()?;
        let expected = Code::XenixRoot;
        assert_eq!(actual, expected);

        let code_str = "0x03";
        let actual: Code = code_str.parse()?;
        let expected = Code::XenixUser;
        assert_eq!(actual, expected);

        let code_str = "0x04";
        let actual: Code = code_str.parse()?;
        let expected = Code::FAT16;
        assert_eq!(actual, expected);

        let code_str = "0x05";
        let actual: Code = code_str.parse()?;
        let expected = Code::ExtendedPartition;
        assert_eq!(actual, expected);

        let code_str = "0x06";
        let actual: Code = code_str.parse()?;
        let expected = Code::FAT16B;
        assert_eq!(actual, expected);

        let code_str = "0x07";
        let actual: Code = code_str.parse()?;
        let expected = Code::HPFSNTFSExfat;
        assert_eq!(actual, expected);

        let code_str = "0x08";
        let actual: Code = code_str.parse()?;
        let expected = Code::AIX;
        assert_eq!(actual, expected);

        let code_str = "0x09";
        let actual: Code = code_str.parse()?;
        let expected = Code::AIXBootable;
        assert_eq!(actual, expected);

        let code_str = "0x0a";
        let actual: Code = code_str.parse()?;
        let expected = Code::OS2BootManager;
        assert_eq!(actual, expected);

        let code_str = "0x0b";
        let actual: Code = code_str.parse()?;
        let expected = Code::W95FAT32;
        assert_eq!(actual, expected);

        let code_str = "0x0c";
        let actual: Code = code_str.parse()?;
        let expected = Code::W95FAT32LBA;
        assert_eq!(actual, expected);

        let code_str = "0x0e";
        let actual: Code = code_str.parse()?;
        let expected = Code::W95FAT16LBA;
        assert_eq!(actual, expected);

        let code_str = "0x0f";
        let actual: Code = code_str.parse()?;
        let expected = Code::W95ExtendedLBA;
        assert_eq!(actual, expected);

        let code_str = "0x10";
        let actual: Code = code_str.parse()?;
        let expected = Code::OPUS;
        assert_eq!(actual, expected);

        let code_str = "0x11";
        let actual: Code = code_str.parse()?;
        let expected = Code::HiddenFAT12;
        assert_eq!(actual, expected);

        let code_str = "0x12";
        let actual: Code = code_str.parse()?;
        let expected = Code::CompaqDiagnostics;
        assert_eq!(actual, expected);

        let code_str = "0x14";
        let actual: Code = code_str.parse()?;
        let expected = Code::HiddenFAT16;
        assert_eq!(actual, expected);

        let code_str = "0x16";
        let actual: Code = code_str.parse()?;
        let expected = Code::HiddenFAT16B;
        assert_eq!(actual, expected);

        let code_str = "0x17";
        let actual: Code = code_str.parse()?;
        let expected = Code::HiddenHPFSNTFSExFat;
        assert_eq!(actual, expected);

        let code_str = "0x18";
        let actual: Code = code_str.parse()?;
        let expected = Code::ASTSmartSleep;
        assert_eq!(actual, expected);

        let code_str = "0x1b";
        let actual: Code = code_str.parse()?;
        let expected = Code::HiddenW95FAT32;
        assert_eq!(actual, expected);

        let code_str = "0x1c";
        let actual: Code = code_str.parse()?;
        let expected = Code::HiddenW95FAT32LBA;
        assert_eq!(actual, expected);

        let code_str = "0x1e";
        let actual: Code = code_str.parse()?;
        let expected = Code::HiddenW95FAT16LBA;
        assert_eq!(actual, expected);

        let code_str = "0x24";
        let actual: Code = code_str.parse()?;
        let expected = Code::NecDOS;
        assert_eq!(actual, expected);

        let code_str = "0x27";
        let actual: Code = code_str.parse()?;
        let expected = Code::HiddenNTFSRescue;
        assert_eq!(actual, expected);

        let code_str = "0x39";
        let actual: Code = code_str.parse()?;
        let expected = Code::Plan9;
        assert_eq!(actual, expected);

        let code_str = "0x3c";
        let actual: Code = code_str.parse()?;
        let expected = Code::PartitionMagic;
        assert_eq!(actual, expected);

        let code_str = "0x40";
        let actual: Code = code_str.parse()?;
        let expected = Code::Venix80286;
        assert_eq!(actual, expected);

        let code_str = "0x41";
        let actual: Code = code_str.parse()?;
        let expected = Code::PPCPrepBoot;
        assert_eq!(actual, expected);

        let code_str = "0x42";
        let actual: Code = code_str.parse()?;
        let expected = Code::Sfs;
        assert_eq!(actual, expected);

        let code_str = "0x4d";
        let actual: Code = code_str.parse()?;
        let expected = Code::QNX4Primary;
        assert_eq!(actual, expected);

        let code_str = "0x4e";
        let actual: Code = code_str.parse()?;
        let expected = Code::QNX4Secondary;
        assert_eq!(actual, expected);

        let code_str = "0x4f";
        let actual: Code = code_str.parse()?;
        let expected = Code::QNX4Tertiary;
        assert_eq!(actual, expected);

        let code_str = "0x50";
        let actual: Code = code_str.parse()?;
        let expected = Code::OnTrackDM;
        assert_eq!(actual, expected);

        let code_str = "0x51";
        let actual: Code = code_str.parse()?;
        let expected = Code::OnTrackDM6Aux1;
        assert_eq!(actual, expected);

        let code_str = "0x52";
        let actual: Code = code_str.parse()?;
        let expected = Code::CPM80;
        assert_eq!(actual, expected);

        let code_str = "0x53";
        let actual: Code = code_str.parse()?;
        let expected = Code::OnTrackDM6Aux3;
        assert_eq!(actual, expected);

        let code_str = "0x54";
        let actual: Code = code_str.parse()?;
        let expected = Code::OnTrackDM6Ddo;
        assert_eq!(actual, expected);

        let code_str = "0x55";
        let actual: Code = code_str.parse()?;
        let expected = Code::EZDrive;
        assert_eq!(actual, expected);

        let code_str = "0x56";
        let actual: Code = code_str.parse()?;
        let expected = Code::GoldenBow;
        assert_eq!(actual, expected);

        let code_str = "0x5c";
        let actual: Code = code_str.parse()?;
        let expected = Code::PriamEDisk;
        assert_eq!(actual, expected);

        let code_str = "0x61";
        let actual: Code = code_str.parse()?;
        let expected = Code::SpeedStor;
        assert_eq!(actual, expected);

        let code_str = "0x63";
        let actual: Code = code_str.parse()?;
        let expected = Code::GNUHurdSystemV;
        assert_eq!(actual, expected);

        let code_str = "0x64";
        let actual: Code = code_str.parse()?;
        let expected = Code::NovellNetware286;
        assert_eq!(actual, expected);

        let code_str = "0x65";
        let actual: Code = code_str.parse()?;
        let expected = Code::NovellNetware386;
        assert_eq!(actual, expected);

        let code_str = "0x70";
        let actual: Code = code_str.parse()?;
        let expected = Code::DiskSecureMultiBoot;
        assert_eq!(actual, expected);

        let code_str = "0x75";
        let actual: Code = code_str.parse()?;
        let expected = Code::PCIX;
        assert_eq!(actual, expected);

        let code_str = "0x80";
        let actual: Code = code_str.parse()?;
        let expected = Code::OldMinix;
        assert_eq!(actual, expected);

        let code_str = "0x81";
        let actual: Code = code_str.parse()?;
        let expected = Code::MinixOldLinux;
        assert_eq!(actual, expected);

        let code_str = "0x82";
        let actual: Code = code_str.parse()?;
        let expected = Code::LinuxSwap;
        assert_eq!(actual, expected);

        let code_str = "0x83";
        let actual: Code = code_str.parse()?;
        let expected = Code::Linux;
        assert_eq!(actual, expected);

        let code_str = "0x84";
        let actual: Code = code_str.parse()?;
        let expected = Code::OS2HiddenCDrive;
        assert_eq!(actual, expected);

        let code_str = "0x85";
        let actual: Code = code_str.parse()?;
        let expected = Code::LinuxExtended;
        assert_eq!(actual, expected);

        let code_str = "0x86";
        let actual: Code = code_str.parse()?;
        let expected = Code::FAT16VolumeSet;
        assert_eq!(actual, expected);

        let code_str = "0x87";
        let actual: Code = code_str.parse()?;
        let expected = Code::NTFSVolumeSet;
        assert_eq!(actual, expected);

        let code_str = "0x88";
        let actual: Code = code_str.parse()?;
        let expected = Code::LinuxPlaintext;
        assert_eq!(actual, expected);

        let code_str = "0x8e";
        let actual: Code = code_str.parse()?;
        let expected = Code::LinuxLVM;
        assert_eq!(actual, expected);

        let code_str = "0x93";
        let actual: Code = code_str.parse()?;
        let expected = Code::Amoeba;
        assert_eq!(actual, expected);

        let code_str = "0x94";
        let actual: Code = code_str.parse()?;
        let expected = Code::AmoebaBadBlockTable;
        assert_eq!(actual, expected);

        let code_str = "0x9f";
        let actual: Code = code_str.parse()?;
        let expected = Code::BSDOs;
        assert_eq!(actual, expected);

        let code_str = "0xa0";
        let actual: Code = code_str.parse()?;
        let expected = Code::IBMThinkpad;
        assert_eq!(actual, expected);

        let code_str = "0xa5";
        let actual: Code = code_str.parse()?;
        let expected = Code::FreeBSD;
        assert_eq!(actual, expected);

        let code_str = "0xa6";
        let actual: Code = code_str.parse()?;
        let expected = Code::OpenBSD;
        assert_eq!(actual, expected);

        let code_str = "0xa7";
        let actual: Code = code_str.parse()?;
        let expected = Code::NextStep;
        assert_eq!(actual, expected);

        let code_str = "0xa8";
        let actual: Code = code_str.parse()?;
        let expected = Code::DarwinUFS;
        assert_eq!(actual, expected);

        let code_str = "0xa9";
        let actual: Code = code_str.parse()?;
        let expected = Code::NetBSD;
        assert_eq!(actual, expected);

        let code_str = "0xab";
        let actual: Code = code_str.parse()?;
        let expected = Code::DarwinBoot;
        assert_eq!(actual, expected);

        let code_str = "0xaf";
        let actual: Code = code_str.parse()?;
        let expected = Code::HFSHFSPlus;
        assert_eq!(actual, expected);

        let code_str = "0xb7";
        let actual: Code = code_str.parse()?;
        let expected = Code::BSDIFs;
        assert_eq!(actual, expected);

        let code_str = "0xb8";
        let actual: Code = code_str.parse()?;
        let expected = Code::BSDISwap;
        assert_eq!(actual, expected);

        let code_str = "0xbb";
        let actual: Code = code_str.parse()?;
        let expected = Code::BootWizardHidden;
        assert_eq!(actual, expected);

        let code_str = "0xbc";
        let actual: Code = code_str.parse()?;
        let expected = Code::AcronisFAT32LBA;
        assert_eq!(actual, expected);

        let code_str = "0xbe";
        let actual: Code = code_str.parse()?;
        let expected = Code::SolarisBoot;
        assert_eq!(actual, expected);

        let code_str = "0xbf";
        let actual: Code = code_str.parse()?;
        let expected = Code::Solaris;
        assert_eq!(actual, expected);

        let code_str = "0xc1";
        let actual: Code = code_str.parse()?;
        let expected = Code::DRDOSSecuredFAT12;
        assert_eq!(actual, expected);

        let code_str = "0xc4";
        let actual: Code = code_str.parse()?;
        let expected = Code::DRDOSSecuredFAT16;
        assert_eq!(actual, expected);

        let code_str = "0xc6";
        let actual: Code = code_str.parse()?;
        let expected = Code::DRDOSSecuredFAT16B;
        assert_eq!(actual, expected);

        let code_str = "0xc7";
        let actual: Code = code_str.parse()?;
        let expected = Code::Syrinx;
        assert_eq!(actual, expected);

        let code_str = "0xda";
        let actual: Code = code_str.parse()?;
        let expected = Code::NonFsData;
        assert_eq!(actual, expected);

        let code_str = "0xdb";
        let actual: Code = code_str.parse()?;
        let expected = Code::CPMCtOs;
        assert_eq!(actual, expected);

        let code_str = "0xde";
        let actual: Code = code_str.parse()?;
        let expected = Code::DellUtilityFAT16;
        assert_eq!(actual, expected);

        let code_str = "0xdf";
        let actual: Code = code_str.parse()?;
        let expected = Code::BootIt;
        assert_eq!(actual, expected);

        let code_str = "0xe1";
        let actual: Code = code_str.parse()?;
        let expected = Code::DOSAccess;
        assert_eq!(actual, expected);

        let code_str = "0xe3";
        let actual: Code = code_str.parse()?;
        let expected = Code::DOSRO;
        assert_eq!(actual, expected);

        let code_str = "0xe4";
        let actual: Code = code_str.parse()?;
        let expected = Code::SpeedStorFAT16;
        assert_eq!(actual, expected);

        let code_str = "0xea";
        let actual: Code = code_str.parse()?;
        let expected = Code::FreedesktopBoot;
        assert_eq!(actual, expected);

        let code_str = "0xeb";
        let actual: Code = code_str.parse()?;
        let expected = Code::BeOSBFS;
        assert_eq!(actual, expected);

        let code_str = "0xee";
        let actual: Code = code_str.parse()?;
        let expected = Code::GPTProtectiveMBR;
        assert_eq!(actual, expected);

        let code_str = "0xef";
        let actual: Code = code_str.parse()?;
        let expected = Code::EfiSystem;
        assert_eq!(actual, expected);

        let code_str = "0xf0";
        let actual: Code = code_str.parse()?;
        let expected = Code::PARISCLinux;
        assert_eq!(actual, expected);

        let code_str = "0xf1";
        let actual: Code = code_str.parse()?;
        let expected = Code::SDSpeedstor;
        assert_eq!(actual, expected);

        let code_str = "0xf4";
        let actual: Code = code_str.parse()?;
        let expected = Code::SpeedStorFAT16B;
        assert_eq!(actual, expected);

        let code_str = "0xf2";
        let actual: Code = code_str.parse()?;
        let expected = Code::DOSSecondary;
        assert_eq!(actual, expected);

        let code_str = "0xf8";
        let actual: Code = code_str.parse()?;
        let expected = Code::EBBRProtective;
        assert_eq!(actual, expected);

        let code_str = "0xfb";
        let actual: Code = code_str.parse()?;
        let expected = Code::VMWareVMFS;
        assert_eq!(actual, expected);

        let code_str = "0xfc";
        let actual: Code = code_str.parse()?;
        let expected = Code::VMWareVMKCORE;
        assert_eq!(actual, expected);

        let code_str = "0xfd";
        let actual: Code = code_str.parse()?;
        let expected = Code::LinuxRaidAuto;
        assert_eq!(actual, expected);

        let code_str = "0xfe";
        let actual: Code = code_str.parse()?;
        let expected = Code::LanStep;
        assert_eq!(actual, expected);

        let code_str = "0xff";
        let actual: Code = code_str.parse()?;
        let expected = Code::XenixBadBlockTable;
        assert_eq!(actual, expected);

        Ok(())
    }
}
