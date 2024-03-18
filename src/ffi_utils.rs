// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Collection of helper functions.

// From dependency library

// From standard library
use std::ffi::{CStr, CString, NulError, OsStr};
use std::fs::File;
use std::io;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::str::Utf8Error;

// From this library

#[doc(hidden)]
#[macro_export]
/// Converts C char* to a Rust `String` or returns an empty String if the pointer is NULL.
macro_rules! ffi_to_string_or_empty {
    ($char_array_ptr:ident) => {
        if $char_array_ptr.is_null() {
            String::new()
        } else {
            $crate::ffi_utils::c_char_array_to_string($char_array_ptr)
        }
    };
}

//---- Conversion functions

#[doc(hidden)]
/// Converts a [`Path`] reference to a [`CString`].
pub fn as_ref_path_to_c_string<T>(path: T) -> Result<CString, NulError>
where
    T: AsRef<Path>,
{
    log::debug!(
        "as_ref_path_to_c_string converting `AsRef<Path>` to `CString`: {:?}",
        path.as_ref()
    );

    CString::new(path.as_ref().as_os_str().as_bytes())
}

#[doc(hidden)]
/// Converts a [`str`](std::str) reference to a [`CString`].
pub fn as_ref_str_to_c_string<T>(string: T) -> Result<CString, NulError>
where
    T: AsRef<str>,
{
    let string: &str = string.as_ref();
    log::debug!(
        "as_ref_str_to_c_string converting `&str` to `CString`: {:?}",
        string
    );

    CString::new(string.as_bytes())
}

#[doc(hidden)]
/// Converts a `const` [`c_char`](libc::c_char) C string to a byte slice.
///
///  # Safety
///
///  - Assumes the  memory pointed to by `ptr` contains a valid nul terminator at the end of the string.
///
///  - `ptr` must be valid for reads of bytes up to and including the null terminator. This means in particular:
///      The entire memory range of the C string must be contained within a single allocated object!
///      `ptr` must be non-null even for a zero-length `cstr`.
///
///  - The memory referenced by the returned CStr must not be mutated for the duration of lifetime 'a.
///
pub fn const_c_char_array_to_bytes<'a>(ptr: *const libc::c_char) -> &'a [u8] {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    log::debug!(
        "const_c_char_array_to_c_str converting `*const libc::c_char` to `[u8]`: {:?}",
        cstr
    );

    cstr.to_bytes()
}

#[doc(hidden)]
/// Converts a [`c_char`](libc::c_char) array to a &[`str`].
pub fn const_char_array_to_str_ref<'a>(ptr: *const libc::c_char) -> Result<&'a str, Utf8Error> {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    log::debug!(
        "c_char_array_to_string converting `*libc::c_char` to `String`: {:?}",
        cstr
    );

    cstr.to_str()
}

#[doc(hidden)]
/// Converts a [`c_char`](libc::c_char) array to a [`String`].
pub fn c_char_array_to_string(ptr: *const libc::c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    log::debug!(
        "c_char_array_to_string converting `*libc::c_char` to `String`: {:?}",
        cstr
    );

    // Get copy-on-write Cow<'_, str>, then guarantee a freshly-owned String allocation
    String::from_utf8_lossy(cstr.to_bytes()).to_string()
}

#[doc(hidden)]
/// Converts a `const` [`c_char`](libc::c_char) C string to a [`Path`].
///
///  # Safety
///
///  - Assumes the  memory pointed to by `ptr` contains a valid nul terminator at the end of the string.
///
///  - `ptr` must be valid for reads of bytes up to and including the null terminator. This means in particular:
///      The entire memory range of the C string must be contained within a single allocated object!
///      `ptr` must be non-null even for a zero-length `cstr`.
///
///  - The memory referenced by the returned CStr must not be mutated for the duration of lifetime 'a.
///
pub fn const_c_char_array_to_path<'a>(ptr: *const libc::c_char) -> &'a Path {
    unsafe {
        log::debug!(
            "const_c_char_array_to_path_buf converting `*const libc::c_char` to `PathBuf`: {:?}",
            CStr::from_ptr(ptr)
        );

        let bytes = CStr::from_ptr(ptr).to_bytes();
        Path::new(OsStr::from_bytes(bytes))
    }
}

#[doc(hidden)]
#[allow(dead_code)]
/// Converts a `const` [`c_char`](libc::c_char) C string to a [`PathBuf`].
///
///  # Safety
///
///  - Assumes the  memory pointed to by `ptr` contains a valid nul terminator at the end of the string.
///
///  - `ptr` must be valid for reads of bytes up to and including the null terminator. This means in particular:
///      The entire memory range of the C string must be contained within a single allocated object!
///      `ptr` must be non-null even for a zero-length `cstr`.
///
///  - The memory referenced by the returned CStr must not be mutated for the duration of lifetime 'a.
///
pub fn const_c_char_array_to_path_buf(ptr: *const libc::c_char) -> PathBuf {
    unsafe {
        log::debug!(
            "const_c_char_array_to_path_buf converting `*const libc::c_char` to `PathBuf`: {:?}",
            CStr::from_ptr(ptr)
        );

        let bytes = CStr::from_ptr(ptr).to_bytes();
        Path::new(OsStr::from_bytes(bytes)).to_path_buf()
    }
}

#[doc(hidden)]
/// Associate a C FILE stream to a `File`'s underlying raw file descriptor.
fn c_file_stream_from(file: &mut File, mode: &CStr) -> io::Result<*mut libc::FILE> {
    unsafe {
        let mut ptr = MaybeUninit::<*mut libc::FILE>::uninit();
        ptr.write(libc::fdopen(file.as_raw_fd(), mode.as_ptr()));

        match ptr.assume_init() {
            ptr if ptr.is_null() => {
                log::debug!("c_file_stream_from fdopen returned a NULL pointer");

                Err(io::Error::last_os_error())
            }
            file_ptr => {
                log::debug!("c_file_stream_from created FILE stream");

                Ok(file_ptr)
            }
        }
    }
}

#[doc(hidden)]
/// Associate a write-only C FILE stream to a `File`'s underlying raw file descriptor.
pub fn read_only_c_file_stream_from(file: &mut File) -> io::Result<*mut libc::FILE> {
    let write_only = CString::new("r")?;
    c_file_stream_from(file, write_only.as_c_str())
}

#[doc(hidden)]
/// Associate a write-only C FILE stream to a `File`'s underlying raw file descriptor.
pub fn write_only_c_file_stream_from(file: &mut File) -> io::Result<*mut libc::FILE> {
    let write_only = CString::new("w")?;
    c_file_stream_from(file, write_only.as_c_str())
}

#[doc(hidden)]
/// Returns the read/write status of an open `File`.
fn is_file_open_read_write(file: &File) -> io::Result<(bool, bool)> {
    const RWMODE: libc::c_int = libc::O_RDONLY | libc::O_RDWR | libc::O_WRONLY;

    unsafe {
        let mode = match libc::fcntl(file.as_raw_fd(), libc::F_GETFL) {
            -1 => {
                log::debug!("is_file_open_read_write failed to get file status flags");
                Err(io::Error::last_os_error())
            }
            status_flags => {
                log::debug!("is_file_open_read_write got file status flags");
                Ok(status_flags)
            }
        }?;

        match mode & RWMODE {
            libc::O_WRONLY => Ok((false, true)),
            libc::O_RDONLY => Ok((true, false)),
            libc::O_RDWR => Ok((true, true)),
            _ => unreachable!("is_file_open_read_write unsupported status flag"),
        }
    }
}

#[doc(hidden)]
/// Returns `true` if a file is open in read-only mode.
pub fn is_open_read_only(file: &File) -> io::Result<bool> {
    let state = is_file_open_read_write(file)? == (true, false);
    log::debug!("is_open_read_only value: {:?}", state);

    Ok(state)
}
