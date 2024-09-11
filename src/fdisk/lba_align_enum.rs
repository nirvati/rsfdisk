// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::IntoPrimitive;

// From standard library
use std::fmt;

// From this library

/// LBA alignment direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
#[allow(dead_code)]
pub(crate) enum LBAAlign {
    Down = libfdisk::FDISK_ALIGN_DOWN,
    Nearest = libfdisk::FDISK_ALIGN_NEAREST,
    Up = libfdisk::FDISK_ALIGN_UP,
}

impl LBAAlign {
    /// View this `LBAAlign` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Down => "down",
            Self::Nearest => "nearest",
            Self::Up => "up",
        }
    }
}

impl AsRef<LBAAlign> for LBAAlign {
    #[inline]
    fn as_ref(&self) -> &LBAAlign {
        self
    }
}

impl fmt::Display for LBAAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
