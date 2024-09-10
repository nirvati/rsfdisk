// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library
use std::ffi::NulError;

// From this library

/// [`PartitionTable`](crate::core::partition_table::PartitionTable) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PartitionTableError {
    /// Error while configuring a new [`PartitionTable`](crate::core::partition_table::PartitionTable) instance.
    #[error("{0}")]
    Config(String),

    /// Error while converting from one type to another.
    #[error("{0}")]
    Conversion(String),

    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("failed to convert value to `CString`: {0}")]
    CStringConversion(#[from] NulError),

    /// Error while parsing a string into a type.
    #[error("{0}")]
    Parse(String),

    /// Error while converting a `u32` to a `PartitionTableKind`.
    #[error("{0}")]
    PartitionTableKind(String),
}
