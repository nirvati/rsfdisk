// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// [`Prompt`](crate::core::prompt::Prompt) runtime errors.
#[derive(Debug, Error, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum PromptError {
    /// Error while allocating memory.
    #[error("{0}")]
    Allocation(String),

    /// Error while configuring a [`Prompt`](crate::core::prompt::Prompt) instance.
    #[error("{0}")]
    Config(String),

    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("{0}")]
    CStringConversion(String),

    /// Error while selecting a [`MenuItem`](crate::core::prompt::MenuItem).
    #[error("{0}")]
    Selection(String),
}
