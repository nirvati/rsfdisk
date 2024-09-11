// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::partition_table::PartitionTable;
use crate::core::private::Sealed;

/// `GPT` specific functions for [`PartitionTable`]s
///
/// This trait is sealed and can not be implemented for types outside of `rsfdisk`.
pub trait PartitionTableGPTExt: Sealed {
    /// Enables the automatic relocation of the `GPT` backup header to the end of the device.
    fn gpt_enable_backup_header_relocation(&mut self);

    /// Disables the automatic relocation of the `GPT` backup header to the end of the device.
    fn gpt_disable_backup_header_relocation(&mut self);

    /// Forces `libfdisk` to write the `GPT` backup header right after the last partition instead
    /// of putting it at the end of the device.
    fn gpt_enable_minimize_footprint(&mut self);

    /// Disables the mechanism forcing `libfdisk` to write the `GPT` backup header right after the
    /// last partition.
    fn gpt_disable_minimize_footprint(&mut self);
}

fn set_relocation(ptr: *mut libfdisk::fdisk_label, enable: bool) {
    let op = if enable { 1 } else { 0 };
    let op_str = if enable {
        "enable".to_owned()
    } else {
        "disable".to_owned()
    };

    unsafe {
        libfdisk::fdisk_gpt_disable_relocation(ptr, op);
    }
    log::debug!(
        "PartitionTableGPTExt::set_relocation {}d automatic backup header relocation",
        op_str
    );
}

fn minimize_footprint(ptr: *mut libfdisk::fdisk_label, enable: bool) {
    let op = if enable { 1 } else { 0 };
    let op_str = if enable {
        "enable".to_owned()
    } else {
        "disable".to_owned()
    };

    unsafe {
        libfdisk::fdisk_gpt_enable_minimize(ptr, op);
    }
    log::debug!(
        "PartitionTableGPTExt::minimize_footprint {}d GPT backup header positioning right after the last partition",
        op_str
    );
}

impl PartitionTableGPTExt for PartitionTable {
    fn gpt_enable_backup_header_relocation(&mut self) {
        log::debug!("PartitionTable::gpt_enable_backup_header_relocation enabling automatic backup header relocation to the end of the device");

        set_relocation(self.inner, true)
    }

    fn gpt_disable_backup_header_relocation(&mut self) {
        log::debug!("PartitionTable::gpt_disable_backup_header_relocation disabling automatic backup header relocation to the end of the device");

        set_relocation(self.inner, false)
    }

    fn gpt_enable_minimize_footprint(&mut self) {
        log::debug!("PartitionTable::gpt_enable_minimize_footprint putting GPT backup header right after last partition");

        minimize_footprint(self.inner, true)
    }

    fn gpt_disable_minimize_footprint(&mut self) {
        log::debug!("PartitionTable::gpt_disable_minimize_footprint putting GPT backup header at the end of the assigned device");

        minimize_footprint(self.inner, false)
    }
}
