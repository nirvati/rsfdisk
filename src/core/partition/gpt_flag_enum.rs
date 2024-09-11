// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::IntoPrimitive;

// From standard library

// From this library

/// `GPT` Partition Entry attribute bits.
///
/// | Bits       | Name                 | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
/// |----       |----                 |----                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
/// | Bit 0      | Required Partition   | If this bit is set, the partition is required for the platform to function. The owner/creator of the partition indicates that deletion or modification of the contents can result in loss of platform features or failure for the platform to boot or operate. The system cannot function normally if this partition is removed, and it should be considered part of the hardware of the system. Actions such as running diagnostics, system recovery, or even OS install or boot could potentially stop working if this partition is removed. Unless OS software or firmware recognizes this partition, it should never be removed or modified as the UEFI firmware or platform hardware may become non-functional. |
/// | Bit 1      | No Block IO Protocol | If this bit is set, then firmware must not produce an EFI_BLOCK_IO_PROTOCOL device for this partition. By not producing an EFI_BLOCK_IO_PROTOCOL partition, file system mappings will not be created for this partition in UEFI ignoring the partition's content.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
/// | Bit 2      | Legacy BIOS Bootable | This bit is set aside by this specification to let systems with traditional PC-AT BIOS firmware implementations inform certain limited, special-purpose software running on these systems that a GPT partition may be bootable. For systems with firmware implementations conforming to this specification, the UEFI boot manager must ignore this bit when selecting a UEFI-compliant application, e.g., an OS loader.                                                                                                                                                                                                                                                                                              |
/// | Bits 3-47  |                      | Undefined and must be zero. Reserved for expansion by future versions of the UEFI specification.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
/// | Bits 48-63 |                      | Reserved for GUID specific use. The use of these bits will vary depending on the PartitionTypeGUID . Only the owner of the PartitionTypeGUID is allowed to modify these bits. They must be preserved if Bits 0-47 are modified.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
///
/// Source: UEFI 2.10 specs, <cite>[Defined GPT Partition Entry - Attributes](https://uefi.org/specs/UEFI/2.10/05_GUID_Partition_Table_Format.html#defined-gpt-partition-entry-attributes)</cite>
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq)]
#[repr(u64)]
#[non_exhaustive]
pub enum GPTFlag {
    /// `Bit 0`.
    RequiredPartition = libfdisk::GPT_FLAG_REQUIRED as u64,

    /// `Bit 1`.
    NoIoBlockProtocol = libfdisk::GPT_FLAG_NOBLOCK as u64,

    /// `Bit 2`.
    LegacyBiosBootable = libfdisk::GPT_FLAG_LEGACYBOOT as u64,

    /// The variant `GuidSpecific` forces `libfdisk` to ask for a bit number through a
    /// [`Prompt`](crate::core::prompt::Prompt).  If you want to toggle a bit in the range [48-63]
    /// without going through a prompt, use the corresponding `Bitxx` enum variant.
    GuidSpecific = libfdisk::GPT_FLAG_GUIDSPECIFIC as u64,

    Bit48 = 48u64,
    Bit49 = 49u64,
    Bit50 = 50u64,
    Bit51 = 51u64,
    Bit52 = 52u64,
    Bit53 = 53u64,
    Bit54 = 54u64,
    Bit55 = 55u64,
    Bit56 = 56u64,
    Bit57 = 57u64,
    Bit58 = 58u64,
    Bit59 = 59u64,
    Bit60 = 60u64,
    Bit61 = 61u64,
    Bit62 = 62u64,
    Bit63 = 63u64,
}
