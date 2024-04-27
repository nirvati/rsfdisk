// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::private::Sealed;
use crate::fdisk::Fdisk;
use crate::fdisk::FdiskError;

/// `GPT` specific functions for a [`Fdisk`].
///
/// This trait is sealed and can not be implemented for types outside of `rsfdisk`.
pub trait FdiskGPTExt: Sealed {
    /// Returns `true` if the `GPT` partition table of the assigned device contains a regular `MBR`
    /// instead of a `PMBR`.
    ///
    /// By default, a `GPT` partition table contains a Protective MBR (`PMBR`) to maintain
    /// compatibility with existing tools that do not understand `GPT` partition structures.
    ///
    /// Contrary to a `PMBR`, a regular `MBR` holds a Partition Entry Array which `libfdisk` does
    /// not keep synchronized with the `GPT` Partition Entry Array. As such, you will need to
    /// perform any required synchronization manually.
    fn gpt_is_hybrid(&self) -> bool;

    /// Returns a partition's attribute bits, or `None` if an error occurred.
    ///
    /// | Bits       | Name                 | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
    /// |----       |----                 |----                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
    /// | Bit 0      | Required Partition   | If this bit is set, the partition is required for the platform to function. The owner/creator of the partition indicates that deletion or modification of the contents can result in loss of platform features or failure for the platform to boot or operate. The system cannot function normally if this partition is removed, and it should be considered part of the hardware of the system. Actions such as running diagnostics, system recovery, or even OS install or boot could potentially stop working if this partition is removed. Unless OS software or firmware recognizes this partition, it should never be removed or modified as the UEFI firmware or platform hardware may become non-functional. |
    /// | Bit 1      | No Block IO Protocol | If this bit is set, then firmware must not produce an EFI_BLOCK_IO_PROTOCOL device for this partition. By not producing an EFI_BLOCK_IO_PROTOCOL partition, file system mappings will not be created for this partition in UEFI ignoring the partition's content.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
    /// | Bit 2      | Legacy BIOS Bootable | This bit is set aside by this specification to let systems with traditional PC-AT BIOS firmware implementations inform certain limited, special-purpose software running on these systems that a GPT partition may be bootable. For systems with firmware implementations conforming to this specification, the UEFI boot manager must ignore this bit when selecting a UEFI-compliant application, e.g., an OS loader.                                                                                                                                                                                                                                                                                              |
    /// | Bits 3-47  |                      | Undefined and must be zero. Reserved for expansion by future versions of the UEFI specification.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
    /// | Bits 48-63 |                      | Reserved for GUID specific use. The use of these bits will vary depending on the PartitionTypeGUID . Only the owner of the PartitionTypeGUID is allowed to modify these bits. They must be preserved if Bits 0-47 are modified.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    fn gpt_attribute_bits(&self, partition_number: usize) -> Option<u64>;

    /// Sets a partition's attribute bits.
    fn gpt_set_attribute_bits<T>(
        &mut self,
        partition_number: usize,
        attribute_bits: u64,
    ) -> Result<(), FdiskError>;

    /// Sets the maximum number of elements in the Partition Entry Array for a `GPT` partition table.
    fn gpt_set_partition_entry_array_size(&mut self, size: u32) -> Result<(), FdiskError>;
}

impl<'a> FdiskGPTExt for Fdisk<'a> {
    fn gpt_is_hybrid(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_gpt_is_hybrid(self.inner) == 1 };
        log::debug!("Fdisk::gpt_is_hybrid value: {:?}", state);

        state
    }

    fn gpt_attribute_bits(&self, partition_number: usize) -> Option<u64> {
        log::debug!(
            "Fdisk::gpt_attribute_bits getting attribute bits for partition {:?}",
            partition_number
        );

        let mut bits = MaybeUninit::<u64>::zeroed();
        let result = unsafe {
            libfdisk::fdisk_gpt_get_partition_attrs(self.inner, partition_number, bits.as_mut_ptr())
        };

        match result {
            0 => {
                let attributes = unsafe { bits.assume_init() };

                log::debug!(
                    "Fdisk::gpt_attribute_bits partition ({}) attribute bits: 0x{:x}",
                    partition_number,
                    attributes
                );

                Some(attributes)
            }
            code => {
                let err_msg = format!(
                    "failed to get attribute bits for partition {:?}",
                    partition_number
                );
                log::debug!("Fdisk::gpt_attribute_bits {}. libfdisk::fdisk_gpt_get_partition_attrs returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    fn gpt_set_attribute_bits<T>(
        &mut self,
        partition_number: usize,
        attribute_bits: u64,
    ) -> Result<(), FdiskError> {
        log::debug!(
            "Fdisk::gpt_set_attribute_bits setting attribute bits 0x{:x} for partition {:?}",
            attribute_bits,
            partition_number
        );

        // Check that bits 3 to 47, reserved for expansion by future versions of the UEFI
        // specification, are not set.
        let mut mask = 1u64 << 4;
        for i in 3..=47 {
            if attribute_bits & mask != 0 {
                let err_msg = format!(
                    "attribute bits 3-47, reserved for future use, must be 0. But, bit {} is set to 1",
                    i
                );
                return Err(FdiskError::Config(err_msg));
            }

            mask <<= 1;
        }

        let result = unsafe {
            libfdisk::fdisk_gpt_set_partition_attrs(self.inner, partition_number, attribute_bits)
        };

        match result {
            0 => {
                log::debug!(
                    "Fdisk::gpt_set_attribute_bits set attribute bits 0x{:x} for partition {:?}",
                    attribute_bits,
                    partition_number
                );

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to set attribute bits 0x{:x} for partition {:?}",
                    attribute_bits, partition_number
                );
                log::debug!("Fdisk::gpt_set_attribute_bits {}. libfdisk::fdisk_gpt_set_partition_attrs returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }

    fn gpt_set_partition_entry_array_size(&mut self, size: u32) -> Result<(), FdiskError> {
        log::debug!("Fdisk::gpt_set_partition_entry_array_size setting GPT partition entry array size to: {:?}", size);

        let result = unsafe { libfdisk::fdisk_gpt_set_npartitions(self.inner, size) };

        match result {
            0 => {
                log::debug!("Fdisk::gpt_set_partition_entry_array_size set GPT partition entry array size to: {:?}", size);

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to set GPT partition entry array size to: {:?}",
                    size
                );
                log::debug!("Fdisk::gpt_set_partition_entry_array_size {}. libfdisk::fdisk_gpt_set_npartitions returned error code: {:?}", err_msg, code);

                Err(FdiskError::Config(err_msg))
            }
        }
    }
}
