// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::partition_table::PartitionTable;
use crate::fdisk::Fdisk;
use crate::owning_mut_from_ptr;

/// Mutable iterator over [`PartitionTable`]s.
#[derive(Debug)]
pub struct PartitionTableIterMut<'a> {
    partitioner: &'a mut Fdisk<'a>,
}

impl<'a> PartitionTableIterMut<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(partitioner: &'a mut Fdisk<'a>) -> PartitionTableIterMut<'a> {
        log::debug!("PartitionTableIterMut::new creating new `PartitionTableIterMut` instance");

        Self { partitioner }
    }
}

impl<'a> Iterator for PartitionTableIterMut<'a> {
    type Item = &'a mut PartitionTable;

    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("PartitionTableIterMut::next getting next partition table");

        let mut table_ptr = MaybeUninit::<*mut libfdisk::fdisk_label>::zeroed();

        let result =
            unsafe { libfdisk::fdisk_next_label(self.partitioner.inner, table_ptr.as_mut_ptr()) };

        match result {
            0 => {
                log::debug!("PartitionTableIterMut::next got next partition table");

                let ptr = unsafe { table_ptr.assume_init() };
                let table = owning_mut_from_ptr!(self.partitioner, PartitionTable, ptr);

                Some(table)
            }
            code => {
                log::debug!("PartitionTableIterMut::next failed to get next partition table. libfdisk::fdisk_next_label returned error code: {:?}", code);

                None
            }
        }
    }
}
