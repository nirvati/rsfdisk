// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// Type conversion runtime errors.
#[derive(Debug, Error, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum ConversionError {
    /// Error while converting bytes into a [`Guid`](crate::core::partition::Guid).
    #[error("{0}")]
    Guid(String),

    /// Error while converting bytes into a [`Code`](crate::core::partition::Code).
    #[error("{0}")]
    Code(String),

    /// Error while converting a value to a [`MaxColWidth`](crate::core::partition_table::MaxColWidth).
    #[error("{0}")]
    MaxColWidth(String),
}
