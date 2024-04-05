// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::partition_table::PartitionTable;
use crate::fdisk::Fdisk;
use crate::owning_ref_from_ptr;

/// Iterator over [`PartitionTable`]s.
#[derive(Debug)]
pub struct PartitionTableIter<'a> {
    partitioner: &'a Fdisk<'a>,
}

impl<'a> PartitionTableIter<'a> {
    pub(crate) fn new(partitioner: &'a Fdisk) -> PartitionTableIter<'a> {
        log::debug!("PartitionTableIter::new creating new `PartitionTableIter` instance");

        Self { partitioner }
    }
}

impl<'a> Iterator for PartitionTableIter<'a> {
    type Item = &'a PartitionTable;

    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("PartitionTableIter::next getting next partition table");

        let mut table_ptr = MaybeUninit::<*mut libfdisk::fdisk_label>::zeroed();

        let result =
            unsafe { libfdisk::fdisk_next_label(self.partitioner.inner, table_ptr.as_mut_ptr()) };

        match result {
            0 => {
                log::debug!("PartitionTableIter::next got next partition table");

                let ptr = unsafe { table_ptr.assume_init() };
                let table = owning_ref_from_ptr!(self.partitioner, PartitionTable, ptr);

                Some(table)
            }
            code => {
                log::debug!("PartitionTableIter::next failed to get next partition table. libfdisk::fdisk_next_label returned error code: {:?}", code);

                None
            }
        }
    }
}
