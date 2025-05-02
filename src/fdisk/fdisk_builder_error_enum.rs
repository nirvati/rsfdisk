// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::fdisk::FdiskError;

/// [`FdiskBuilder`](crate::fdisk::FdiskBuilder) runtime errors.
#[derive(Debug, Error, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum FdiskBuilderError {
    /// Error while configuring `Fdisk` instance.
    #[error(transparent)]
    Config(#[from] FdiskError),

    /// Error if two mutually exclusive setter functions are called.
    #[error("{0}")]
    MutuallyExclusive(String),

    /// Error if a required function was not called.
    #[error("{0}")]
    Required(String),
}
