// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library
use std::ffi::NulError;

// From this library

/// [`Fdisk`](crate::fdisk::Fdisk) runtime errors.
#[derive(Debug, Error)]
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

    /// Error while creating a new [`Fdisk`](crate::fdisk::Fdisk) instance.
    #[error("{0}")]
    Creation(String),

    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("failed to convert value to `CString`: {}", .0)]
    CStringConversion(#[from] NulError),

    /// Input/Output runtime errors.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Error while printing log messages.
    #[error("{0}")]
    Log(String),

    /// Error while reading the answer to a prompt.
    #[error("{0}")]
    Prompt(String),
}
