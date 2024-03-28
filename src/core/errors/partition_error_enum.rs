// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library
use std::ffi::NulError;

// From this library
use crate::core::errors::PartitionKindError;

/// [`Partition`](crate::core::partition::Partition) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PartitionError {
    /// Error while creating a new [`Partition`](crate::core::partition::Partition) instance.
    #[error("{0}")]
    Creation(String),

    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("failed to convert value to `CString`: {0}")]
    CStringConversion(#[from] NulError),

    #[error(transparent)]
    PartitionKind(#[from] PartitionKindError),

    /// Error while configuring a new [`Partition`](crate::core::partition::Partition) instance.
    #[error("{0}")]
    Config(String),
}
