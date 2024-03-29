// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library
use std::ffi::NulError;

// From this library

/// [`PartitionKind`](crate::core::partition::PartitionKind) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PartitionKindError {
    /// Error while copying a [`PartitionKind`](crate::core::partition::PartitionKind).
    #[error("{0}")]
    Copy(String),

    /// Error while creating a new [`PartitionKind`](crate::core::partition::PartitionKind) instance.
    #[error("{0}")]
    Creation(String),

    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("failed to convert value to `CString`: {0}")]
    CStringConversion(#[from] NulError),

    /// Error while configuring a new [`PartitionKind`](crate::core::partition::PartitionKind) instance.
    #[error("{0}")]
    Setting(String),
}
