// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

/// Type of partition.
///
/// There are two families of partitions GUID/GPT-based amd MBR-based. GUID/GPT partitions are
/// identified by a 16-byte string UUID, while MBR partitions are by a 1-byte hexadecimal
/// code.
#[derive(Debug)]
#[repr(transparent)]
pub struct PartitionKind {
    pub(crate) inner: *mut libfdisk::fdisk_parttype,
}

impl Drop for PartitionKind {
    fn drop(&mut self) {
        log::debug!("PartitionKind::drop deallocating `PartitionKind` instance");

        unsafe { libfdisk::fdisk_unref_parttype(self.inner) }
    }
}
