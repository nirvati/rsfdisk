// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library
use std::ffi::NulError;

// From this library

/// [`Fdisk`](crate::fdisk::Fdisk) runtime errors.
#[derive(Debug, Error, Clone)]
#[non_exhaustive]
pub enum FdiskError {
    /// Error while assigning a device to a [`Fdisk`](crate::fdisk::Fdisk) instance.
    #[error("{0}")]
    AssignDevice(String),

    /// Error while closing the device assigned to a [`Fdisk`](crate::fdisk::Fdisk) instance.
    #[error("{0}")]
    CloseDevice(String),

    /// Error while configuring a [`Fdisk`](crate::fdisk::Fdisk).
    #[error("{0}")]
    Config(String),

    /// Error while performing a conversion.
    #[error("{0}")]
    Conversion(String),

    /// Error while creating a new [`Fdisk`](crate::fdisk::Fdisk) instance.
    #[error("{0}")]
    Creation(String),

    /// Error when prompts are disabled.
    #[error("{0}")]
    DialogsDisabled(String),

    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("failed to convert value to `CString`: {}", .0)]
    CStringConversion(#[from] NulError),

    /// Error while aligning data to block sector boundaries on disk.
    #[error("{0}")]
    DataAlignment(String),

    /// Input/Output runtime errors.
    #[error("{0}")]
    IoError(String),

    /// Error while printing log messages.
    #[error("{0}")]
    Log(String),

    /// Error when there is no more assignable partition number.
    #[error("{0}")]
    NoNextPartitionNumber(String),

    /// Error when trying to allocate memory.
    #[error("{0}")]
    OutOfMemory(String),

    /// Error while overriding `Fdisk` attributes in memory.
    #[error("{0}")]
    Override(String),

    /// Error while reading the answer to a prompt.
    #[error("{0}")]
    Prompt(String),

    /// Error while restoring `Fdisk` attributes from their saved values on disk.
    #[error("{0}")]
    Restore(String),

    /// Error if returned value is out of range.
    #[error("{0}")]
    ResultOutOfRange(String),

    /// Error while saving `Fdisk` attributes.
    #[error("{0}")]
    Save(String),

    /// [`Script`](crate::core::script::Script) runtime error.
    #[error("{0}")]
    Script(String),

    #[error("{0}")]
    Unexpected(String),
}
