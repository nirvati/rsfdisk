// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::core::errors::PartitionError;

/// [`PartitionBuilder`](crate::core::partition::PartitionBuilder) runtime errors.
#[derive(Debug, Error, Clone)]
#[non_exhaustive]
pub enum PartitionBuilderError {
    /// Error while configuring [`Partition`](crate::core::partition::Partition) instance.
    #[error(transparent)]
    Config(#[from] PartitionError),

    /// Error if two mutually exclusive setter functions are called.
    #[error("{0}")]
    MutuallyExclusive(String),
}
