// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::partition_table::PartitionTable;
use crate::fdisk::Fdisk;

pub trait Sealed {}

impl Sealed for PartitionTable {}
impl<'a> Sealed for Fdisk<'a> {}
