// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

/// Partition metadata.
#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Partition {
    pub(crate) inner: *mut libfdisk::fdisk_partition,
}

impl AsRef<Partition> for Partition {
    #[inline]
    fn as_ref(&self) -> &Partition {
        self
    }
}

impl Drop for Partition {
    fn drop(&mut self) {
        log::debug!("Partition::drop deallocating `Partition` instance");

        unsafe { libfdisk::fdisk_unref_partition(self.inner) }
    }
}
