// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::core::errors::PartitionKindError;

/// [`PartitionKindBuilder`](crate::core::partition::PartitionKindBuilder) runtime errors.
#[derive(Debug, Error, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum PartitionKindBuilderError {
    /// Error while configuring [`PartitionKind`](crate::core::partition::PartitionKind) instance.
    #[error(transparent)]
    Config(#[from] PartitionKindError),

    /// Error if two mutually exclusive setter functions are called.
    #[error("{0}")]
    MutuallyExclusive(String),

    /// Error if a required method is not called.
    #[error("{0}")]
    Required(String),
}
