// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::{IntoPrimitive, TryFromPrimitive};

// From standard library

// From this library

/// [`GenIterator`](crate::core::iter::GenIterator)'s direction of iteration.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum Direction {
    Forward = libfdisk::FDISK_ITER_FORWARD as i32,
    Backward = libfdisk::FDISK_ITER_BACKWARD as i32,
}
