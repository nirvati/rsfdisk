// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::errors::HeaderEntryContentError;
use crate::ffi_utils;

/// Content of an entry in a Partition Table Header.
#[derive(Debug)]
#[repr(transparent)]
pub struct HeaderEntryContent {
    pub(crate) inner: *mut libfdisk::fdisk_labelitem,
}

impl HeaderEntryContent {
    #[doc(hidden)]
    /// Increments a `HeaderEntryContent`'s reference counter.
    pub(crate) fn incr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_ref_labelitem(self.inner) }
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Decrements a `HeaderEntryContent`'s reference counter.
    pub(crate) fn decr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_unref_labelitem(self.inner) }
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Borrows a `HeaderEntryContent` instance.
    pub(crate) fn borrow_ptr(ptr: *mut libfdisk::fdisk_labelitem) -> HeaderEntryContent {
        let mut partition = Self::from_ptr(ptr);
        // We are virtually ceding ownership of this partition which will be automatically
        // deallocated once it is out of scope, incrementing its reference counter protects it from
        // being freed prematurely.
        partition.incr_ref_counter();

        partition
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Wraps a raw `libfdisk::fdisk_labelitem` with a safe `HeaderEntryContent`.
    pub(crate) fn from_ptr(ptr: *mut libfdisk::fdisk_labelitem) -> HeaderEntryContent {
        Self { inner: ptr }
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Creates a new `HeaderEntryContent`.
    pub(crate) fn new() -> Result<HeaderEntryContent, HeaderEntryContentError> {
        log::debug!("HeaderEntryContent::new creating a new `HeaderEntryContent` instance");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_labelitem>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_new_labelitem());
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `HeaderEntryContent` instance".to_owned();
                log::debug!(
                    "HeaderEntryContent::new {}. libfdisk::fdisk_new_labelitem returned a NULL pointer",
                    err_msg
                );

                Err(HeaderEntryContentError::Creation(err_msg))
            }
            ptr => {
                log::debug!("HeaderEntryContent::new created a new `HeaderEntryContent` instance");
                let entry = Self::from_ptr(ptr);

                Ok(entry)
            }
        }
    }

    /// Returns the name of an entry in a partition table header.
    pub fn name(&self) -> Option<&str> {
        log::debug!("HeaderEntryContent::name getting name of partition table header entry");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_labelitem_get_name(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("HeaderEntryContent::name no name found for partition table header entry. libfdisk::fdisk_labelitem_get_name returned a NULL pointer");

                None
            }
            name_ptr => {
                let name = ffi_utils::const_char_array_to_str_ref(name_ptr).ok();
                log::debug!("HeaderEntryContent::name value: {:?}", name);

                name
            }
        }
    }

    // FIXME see how can we convert from the returned i32 to a HeaderEntry with the overlap
    // between values as currently defined
    /// Returns the type of header entry this `HeaderEntryContent` belongs to.
    pub fn header_entry(&self) -> i32 {
        //HeaderEntryKind::try_from(id).unwrap()
        let id = unsafe { libfdisk::fdisk_labelitem_get_id(self.inner) };
        log::debug!(
            "HeaderEntryContent::kind partition table entry ID: {:?}",
            id
        );

        id
    }

    /// Returns the `u64` data in a partition table header entry.
    pub fn data_u64(&self) -> Option<u64> {
        log::debug!(
            "HeaderEntryContent::data_u64 getting `u64` data from partition table header entry"
        );

        let mut ptr = MaybeUninit::<u64>::zeroed();

        let result =
            unsafe { libfdisk::fdisk_labelitem_get_data_u64(self.inner, ptr.as_mut_ptr()) };

        match result {
            0 => {
                log::debug!("HeaderEntryContent::data_u64 got u64 data");

                let data = unsafe { ptr.assume_init() };
                Some(data)
            }
            code => {
                let err_msg = "failed to get u64 data".to_owned();
                log::debug!("HeaderEntryContent::data_u64 {}. libfdisk::fdisk_labelitem_get_data_u64 returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    /// Returns the `String` data in a partition table header entry.
    pub fn data_string(&self) -> Option<&str> {
        log::debug!(
            "HeaderEntryContent::data_u64 getting `String` data from partition table header entry"
        );

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();

        let result =
            unsafe { libfdisk::fdisk_labelitem_get_data_string(self.inner, ptr.as_mut_ptr()) };

        match result {
            0 => match unsafe { ptr.assume_init() } {
                ptr if ptr.is_null() => {
                    let err_msg = "no string data in header entry".to_owned();
                    log::debug!("HeaderEntryContent::data_string {}. libfdisk::fdisk_labelitem_get_data_string returned a NULL pointer", err_msg);

                    None
                }
                ptr => {
                    let string = ffi_utils::const_char_array_to_str_ref(ptr).ok();
                    log::debug!("HeaderEntryContent::data_string value: {:?}", string);

                    string
                }
            },
            code => {
                let err_msg =
                    "failed to get string data from partition table header entry".to_owned();
                log::debug!("HeaderEntryContent::data_string {}. libfdisk::fdisk_labelitem_get_data_string returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    /// Returns `true` if the value contained in this `HeaderEntryContent` is a string.
    pub fn is_string(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_labelitem_is_string(self.inner) == 1 };
        log::debug!("HeaderEntryContent::is_string value: {:?}", state);

        state
    }

    /// Returns `true` if the value contained in this `HeaderEntryContent` is a number.
    pub fn is_numeric(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_labelitem_is_number(self.inner) == 1 };
        log::debug!("HeaderEntryContent::is_numeric value: {:?}", state);

        state
    }
}

impl AsRef<HeaderEntryContent> for HeaderEntryContent {
    #[inline]
    fn as_ref(&self) -> &HeaderEntryContent {
        self
    }
}

impl Drop for HeaderEntryContent {
    fn drop(&mut self) {
        log::debug!("HeaderEntryContent::drop deallocating `HeaderEntryContent` instance");

        unsafe { libfdisk::fdisk_unref_labelitem(self.inner) }
    }
}
