// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::IntoPrimitive;

// From standard library

// From this library

/// Types of input for the generic partition type parser.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq)]
#[repr(u32)]
#[non_exhaustive]
pub enum InputType {
    /// Assume input is a type alias (e.g `linux` for a linux partition).
    Alias = libfdisk::fdisk_parttype_parser_flags_FDISK_PARTTYPE_PARSE_ALIAS,

    /// Combination of recommended flags (`HexOrUuid`, `Shorcut`, `Alias`, `Name`, and
    /// `SequenceNumber`).
    Default = libfdisk::fdisk_parttype_parser_flags_FDISK_PARTTYPE_PARSE_DEFAULT,

    /// Accept deprecated aliases and shortcuts.
    Deprecated = libfdisk::fdisk_parttype_parser_flags_FDISK_PARTTYPE_PARSE_DEPRECATED,

    /// Assume input is a number in hexadecimal or a UUID.
    HexOrUuid = libfdisk::fdisk_parttype_parser_flags_FDISK_PARTTYPE_PARSE_DATA,

    /// Assume input is the name of a partition type in human readable form.
    Name = libfdisk::fdisk_parttype_parser_flags_FDISK_PARTTYPE_PARSE_NAME,

    /// Ignore unknown types.
    IgnoreUnknown = libfdisk::fdisk_parttype_parser_flags_FDISK_PARTTYPE_PARSE_NOUNKNOWN,

    /// Assume input is a sequence number as displayed by `fdisk` in the console.
    SequenceNumber = libfdisk::fdisk_parttype_parser_flags_FDISK_PARTTYPE_PARSE_SEQNUM,

    /// Assume input is a shortcut (i.e. `L` for linux partitions).
    Shortcut = libfdisk::fdisk_parttype_parser_flags_FDISK_PARTTYPE_PARSE_SHORTCUT,
}
