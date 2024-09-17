// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::partition::DOSFlag;
use crate::core::partition::GPTFlag;
use crate::core::partition::SGIFlag;

/// Partition flags by type of partition table.
#[derive(Debug)]
#[non_exhaustive]
pub enum BitFlag {
    DOS(DOSFlag),
    GPT(GPTFlag),
    SGI(SGIFlag),
}

impl BitFlag {
    /// Converts an `BitFlag` to its underling `u64` value.
    pub fn to_u64(&self) -> u64 {
        match self {
            Self::DOS(flag) => *flag as u64,
            Self::GPT(bit) => *bit as u64,
            Self::SGI(flag) => *flag as u64,
        }
    }
}

impl From<DOSFlag> for BitFlag {
    #[inline]
    fn from(flag: DOSFlag) -> BitFlag {
        Self::DOS(flag)
    }
}

impl From<GPTFlag> for BitFlag {
    #[inline]
    fn from(bit: GPTFlag) -> BitFlag {
        Self::GPT(bit)
    }
}

impl From<SGIFlag> for BitFlag {
    #[inline]
    fn from(flag: SGIFlag) -> BitFlag {
        Self::SGI(flag)
    }
}
