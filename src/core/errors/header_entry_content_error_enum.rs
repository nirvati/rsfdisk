// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// [`HeaderEntryContent`](crate::core::partition_table::HeaderEntryContent) runtime errors.
#[derive(Debug, Error, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum HeaderEntryContentError {
    /// Error while creating a new [`HeaderEntryContent`](crate::core::partition_table::HeaderEntryContent) instance.
    #[error("{0}")]
    Creation(String),
}
