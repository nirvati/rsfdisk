// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::IntoPrimitive;

// From standard library

// From this library

/// `SGI` partition flags.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq)]
#[repr(u64)]
#[non_exhaustive]
pub enum SGIFlag {
    Boot = libfdisk::SGI_FLAG_BOOT as u64,
    Swap = libfdisk::SGI_FLAG_SWAP as u64,
}
