// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::fdisk::GcItem;

/// Partition table reader/editor/creator.
///
/// `Fdisk` is the main avenue for reading, editing, and creating partition tables on a block
/// device, either a hard disk or a disk image.
#[derive(Debug)]
pub struct Fdisk<'a> {
    pub(crate) inner: *mut libfdisk::fdisk_context,
    _parent: Option<&'a Fdisk<'a>>,
    pub(crate) gc: Vec<GcItem>,
}

impl<'a> Drop for Fdisk<'a> {
    fn drop(&mut self) {
        log::debug!("Fdisk::drop deallocating `Fdisk` instance");

        unsafe { libfdisk::fdisk_unref_context(self.inner) }

        // Release heap allocated PartitionTable references.
        while let Some(gc_item) = self.gc.pop() {
            gc_item.destroy();
        }
    }
}
