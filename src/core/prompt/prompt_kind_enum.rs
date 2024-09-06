// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::{IntoPrimitive, TryFromPrimitive};

// From standard library

// From this library

/// Kinds of prompt displayed by an `Fdisk` object.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum PromptKind {
    /// Print an information message.
    Info = libfdisk::fdisk_asktype_FDISK_ASKTYPE_INFO,

    /// Display a choice menu.
    Menu = libfdisk::fdisk_asktype_FDISK_ASKTYPE_MENU,

    /// Undefined.
    None = libfdisk::fdisk_asktype_FDISK_ASKTYPE_NONE,

    /// Prompt for a number.
    Number = libfdisk::fdisk_asktype_FDISK_ASKTYPE_NUMBER,

    /// Prompt for an offset.
    Offset = libfdisk::fdisk_asktype_FDISK_ASKTYPE_OFFSET,

    /// Prompt for a string value.
    String = libfdisk::fdisk_asktype_FDISK_ASKTYPE_STRING,

    /// Print an error message and a error number (errno).
    Warn = libfdisk::fdisk_asktype_FDISK_ASKTYPE_WARN,

    /// Print a warning message.
    WarnX = libfdisk::fdisk_asktype_FDISK_ASKTYPE_WARNX,

    /// Ask a yes/no question.
    YesNo = libfdisk::fdisk_asktype_FDISK_ASKTYPE_YESNO,
}
