// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum GcItem {
    Partition(*mut *mut libfdisk::fdisk_partition),
    PartitionTable(*mut *mut libfdisk::fdisk_label),
}

impl GcItem {
    #[doc(hidden)]
    #[allow(dead_code)]
    /// Consumes the `GcItem` and frees the memory it points to.
    pub(crate) fn destroy(self) {
        match self {
            Self::Partition(boxed_ptr) => {
                let _ = unsafe { Box::from_raw(boxed_ptr) };
            }
            Self::PartitionTable(boxed_ptr) => {
                let _ = unsafe { Box::from_raw(boxed_ptr) };
            }
        }
    }
}

impl From<*mut *mut libfdisk::fdisk_partition> for GcItem {
    fn from(ptr: *mut *mut libfdisk::fdisk_partition) -> GcItem {
        Self::Partition(ptr)
    }
}

impl From<*mut *mut libfdisk::fdisk_label> for GcItem {
    fn from(ptr: *mut *mut libfdisk::fdisk_label) -> GcItem {
        Self::PartitionTable(ptr)
    }
}
