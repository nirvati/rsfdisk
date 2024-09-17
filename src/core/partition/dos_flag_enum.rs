// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::IntoPrimitive;

// From standard library

// From this library

/// `DOS` partition flag.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq)]
#[repr(u64)]
#[non_exhaustive]
pub enum DOSFlag {
    /// Indicates that the partition is a bootable legacy partition.
    Boot = libfdisk::DOS_FLAG_ACTIVE as u64,
}
