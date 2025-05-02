// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// [`PartitionList`](crate::core::partition::PartitionList) runtime errors.
#[derive(Debug, Error, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum PartitionListError {
    /// Error while removing all partitions in a [`PartitionList`](crate::core::partition::PartitionList).
    #[error("{0}")]
    Clear(String),

    /// Error while creating a new [`PartitionList`](crate::core::partition::PartitionList) instance.
    #[error("{0}")]
    Creation(String),

    /// Error while indexing entries in [`PartitionList`](crate::core::partition::PartitionList).
    #[error("{0}")]
    IndexOutOfBounds(String),

    /// Error while adding a partition to a [`PartitionList`](crate::core::partition::PartitionList).
    #[error("{0}")]
    Push(String),

    /// Error while sorting a [`PartitionList`](crate::core::partition::PartitionList).
    #[error("{0}")]
    Sort(String),
}
