// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Collection of helper functions.

// From dependency library

// From standard library
use std::ffi::{CStr, CString, NulError};
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
