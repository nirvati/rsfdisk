// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// [`Script`](crate::core::script::Script) runtime errors.
#[derive(Debug, Error, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum ScriptError {
    /// Error while configuring a [`Script`](crate::core::script::Script).
    #[error("{0}")]
    Config(String),

    /// Error while composing a [`Script`](crate::core::script::Script).
    #[error("{0}")]
    Compose(String),

    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("{0}")]
    CStringConversion(String),

    /// Error while reading a [`Script`](crate::core::script::Script) from a file.
    #[error("{0}")]
    Read(String),

    /// Input/Output runtime error.
    #[error("{0}")]
    IoError(String),

    /// Error while overriding a value in a [`Script`](crate::core::script::Script).
    #[error("{0}")]
    Override(String),

    /// Error while writing a [`Script`](crate::core::script::Script) to a file.
    #[error("{0}")]
    Write(String),
}
