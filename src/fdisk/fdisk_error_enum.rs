// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// [`Fdisk`](crate::fdisk::Fdisk) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FdiskError {}
