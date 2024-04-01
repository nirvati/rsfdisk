// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::errors::GenIteratorError;
use crate::core::iter::Direction;

/// Generic iterator.
#[derive(Debug)]
pub struct GenIterator {
    pub(crate) inner: *mut libfdisk::fdisk_iter,
}

impl GenIterator {
    /// Creates a new `GenIterator` instance.
    pub fn new(direction: Direction) -> Result<GenIterator, GenIteratorError> {
        log::debug!("GenIterator::new creating a new `GenIterator` instance");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_iter>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_new_iter(direction as i32));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `GenIterator` instance".to_owned();
                log::debug!(
                    "GenIterator::new {}. libfdisk::fdisk_new_iter returned a NULL pointer",
                    err_msg
                );

                Err(GenIteratorError::Creation(err_msg))
            }
            ptr => {
                log::debug!("GenIterator::new created a new `GenIterator` instance");

                let iterator = Self { inner: ptr };

                Ok(iterator)
            }
        }
    }

    /// Returns the [`Direction`] of iteration.
    pub fn direction(&self) -> Direction {
        let code = unsafe { libfdisk::fdisk_iter_get_direction(self.inner) };
        let direction = Direction::try_from(code).unwrap();

        log::debug!(
            "GenIterator::direction direction of iteration: {:?}",
            direction
        );

        direction
    }

    /// Resets the position of the next element in the collection to that of the
    /// first element. This method keeps the [`Direction`] of iteration unchanged.
    pub fn reset(&self) {
        log::debug!("GenIterator::reset resetting iterator with direction unchanged");
        const UNCHANGED_DIRECTION: libc::c_int = -1;

        unsafe { libfdisk::fdisk_reset_iter(self.inner, UNCHANGED_DIRECTION) }
    }

    /// Resets the position of the next element in the collection to that of the
    /// first element, and sets the [`Direction`] of iteration to [`Direction::Forward`].
    pub fn reset_forward(&self) {
        log::debug!(
            "GenIterator::reset_forward resetting iterator, setting direction: {:?}",
            Direction::Forward
        );
        let direction = Direction::Forward;

        unsafe { libfdisk::fdisk_reset_iter(self.inner, direction as i32) }
    }

    /// Resets the position of the next element in the collection to that of the
    /// first element, and sets the [`Direction`] of iteration to [`Direction::Backward`].
    pub fn reset_backward(&self) {
        log::debug!(
            "GenIterator::reset_backward resetting iterator, setting direction: {:?}",
            Direction::Backward
        );
        let direction = Direction::Backward;

        unsafe { libfdisk::fdisk_reset_iter(self.inner, direction as i32) }
    }
}

impl Drop for GenIterator {
    fn drop(&mut self) {
        log::debug!("GenIterator::drop deallocating `GenIterator` instance");

        unsafe { libfdisk::fdisk_free_iter(self.inner) }
    }
}
