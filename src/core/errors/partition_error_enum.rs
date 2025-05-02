// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::core::errors::PartitionKindError;

/// [`Partition`](crate::core::partition::Partition) runtime errors.
#[derive(Debug, Error, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum PartitionError {
    /// Error while creating a new [`Partition`](crate::core::partition::Partition) instance.
    #[error("{0}")]
    Creation(String),

    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("{0}")]
    CStringConversion(String),

    #[error(transparent)]
    PartitionKind(#[from] PartitionKindError),

    /// Error while configuring a new [`Partition`](crate::core::partition::Partition) instance.
    #[error("{0}")]
    Config(String),
}
