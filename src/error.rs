// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Library-level error module.

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::GenIteratorError;
use crate::core::errors::HeaderEntryContentError;
use crate::core::errors::ParserError;
use crate::core::errors::PartitionBuilderError;
use crate::core::errors::PartitionError;
use crate::core::errors::PartitionIterError;
use crate::core::errors::PartitionKindBuilderError;
use crate::core::errors::PartitionKindError;
use crate::core::errors::PartitionListError;
use crate::core::errors::PartitionTableError;
use crate::core::errors::PromptError;
use crate::core::errors::ScriptError;

use crate::fdisk::FdiskBuilderError;
use crate::fdisk::FdiskError;

use crate::core::utils::version::VersionError;

/// A specialized [`Result`](std::result::Result) type for `rsfdisk`.
///
/// This typedef is generally used at the program-level to avoid writing out [`RsFdiskError`]
/// directly, and is, otherwise, a direct mapping to [`Result`](std::result::Result).
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, RsFdiskError>;

/// Library-level runtime errors.
///
/// This enum includes all variants of error types susceptible to occur in the library. Other, more
/// granular error types, are automatically converted to an `RsFdiskError` when needed.
///
/// # Examples
/// ----
///
/// ```
/// fn main() -> rsfdisk::Result<()> {
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Error, Clone)]
#[non_exhaustive]
pub enum RsFdiskError {
    #[error(transparent)]
    Conversion(#[from] ConversionError),

    #[error(transparent)]
    Fdisk(#[from] FdiskError),

    #[error(transparent)]
    FdiskBuilder(#[from] FdiskBuilderError),

    #[error(transparent)]
    GenIterator(#[from] GenIteratorError),

    #[error(transparent)]
    HeaderEntryContent(#[from] HeaderEntryContentError),

    #[error(transparent)]
    Parser(#[from] ParserError),

    #[error(transparent)]
    Partition(#[from] PartitionError),

    #[error(transparent)]
    PartitionBuilder(#[from] PartitionBuilderError),

    #[error(transparent)]
    PartitionIter(#[from] PartitionIterError),

    #[error(transparent)]
    PartitionKind(#[from] PartitionKindError),

    #[error(transparent)]
    PartitionKindBuilder(#[from] PartitionKindBuilderError),

    #[error(transparent)]
    PartitionList(#[from] PartitionListError),

    #[error(transparent)]
    PartitionTable(#[from] PartitionTableError),

    #[error(transparent)]
    Prompt(#[from] PromptError),

    #[error(transparent)]
    Script(#[from] ScriptError),

    #[error(transparent)]
    Version(#[from] VersionError),
}
