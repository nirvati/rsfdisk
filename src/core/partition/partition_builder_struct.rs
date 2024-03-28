// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use typed_builder::TypedBuilder;

// From standard library

// From this library
use crate::core::errors::PartitionBuilderError;

use crate::core::partition::Partition;
use crate::core::partition::PartitionKind;

#[derive(Debug, TypedBuilder)]
#[builder(
    builder_type(
        name = PartitionBuilder,
        vis = "pub",
        doc ="Configure and instantiate a [`Partition`].\n\nFor usage, see [`PartitionBuilder::build`]."),
    build_method(vis = "", name = __make))]
pub(crate) struct PartBuilder {
    #[builder(setter(
        strip_bool,
        doc = "Call this function if you want to be able to set a different partition size when
using a [`Partition`] instance as a template to add new partition in a partition table."
    ))]
    ask_size_interactive: bool,

    #[builder(default, setter(transform = |bits: impl AsRef<[u8]>| Some(bits.as_ref().to_owned()),
    doc = "Set the partition attributes."))]
    attribute_bits: Option<Vec<u8>>,

    #[builder(default, setter(strip_option, doc = "Set the partition's type."))]
    partition_type: Option<PartitionKind>,

    #[builder(default, setter(into, strip_option, doc = "Set the partition's name."))]
    name: Option<String>,

    #[builder(
        default,
        setter(
            strip_option,
            doc = "Set the partition's identification number. By default, set to
the first free partition number available."
        )
    )]
    number: Option<usize>,

    #[builder(
        default,
        setter(
            strip_option,
            doc = "Set the partition's size in sectors. By default, takes all the
free space up to the last free sector."
        )
    )]
    size_in_sectors: Option<u64>,

    #[builder(
        default,
        setter(
            strip_option,
            doc = "Set the offset of the partition's first sector with respect to the beginning of
the device. By default, set to the first available free sector."
        )
    )]
    starting_sector: Option<u64>,

    #[builder(default, setter(into, strip_option, doc = "Set the partition's UUID"))]
    uuid: Option<String>,
}

#[allow(non_camel_case_types)]
impl<
        __ask_size_interactive: ::typed_builder::Optional<bool>,
        __attribute_bits: ::typed_builder::Optional<Option<Vec<u8>>>,
        __partition_type: ::typed_builder::Optional<Option<PartitionKind>>,
        __name: ::typed_builder::Optional<Option<String>>,
        __number: ::typed_builder::Optional<Option<usize>>,
        __size_in_sectors: ::typed_builder::Optional<Option<u64>>,
        __starting_sector: ::typed_builder::Optional<Option<u64>>,
        __uuid: ::typed_builder::Optional<Option<String>>,
    >
    PartitionBuilder<(
        __ask_size_interactive,
        __attribute_bits,
        __partition_type,
        __name,
        __number,
        __size_in_sectors,
        __starting_sector,
        __uuid,
    )>
{
    /// Completes a [`Partition`]'s configuration process, and creates a new instance.
    pub fn build(self) -> Result<Partition, PartitionBuilderError> {
        let builder = self.__make();
        let mut partition = Partition::new()?;

        if let Some(partition_type) = builder.partition_type {
            partition.set_partition_type(partition_type)?;
        }

        if builder.ask_size_interactive {
            partition.ask_size_interactive()?;
        }

        if let Some(bits) = builder.attribute_bits {
            partition.set_attribute_bits(bits)?;
        }

        if let Some(name) = builder.name {
            partition.set_name(name)?;
        }

        // Setting partition identification number.
        match builder.number {
            Some(number) => partition
                .set_partition_number(number)
                .map_err(PartitionBuilderError::from),
            None => partition
                .use_first_free_partition_number(true)
                .map_err(PartitionBuilderError::from),
        }?;

        // Setting partition starting sector.
        match builder.starting_sector {
            Some(starting_sector) => partition
                .set_starting_sector(starting_sector)
                .map_err(PartitionBuilderError::from),
            None => partition
                .use_first_free_starting_sector(true)
                .map_err(PartitionBuilderError::from),
        }?;

        // Setting partition size.
        match builder.size_in_sectors {
            Some(size) => {
                // Do not set the last free sector as the end of the partition.
                partition
                    .use_last_free_ending_sector(false)
                    .map_err(PartitionBuilderError::from)?;

                partition
                    .set_size_in_sectors(size)
                    .map_err(PartitionBuilderError::from)
            }
            None => partition
                .use_last_free_ending_sector(true)
                .map_err(PartitionBuilderError::from),
        }?;

        if let Some(uuid) = builder.uuid {
            partition.set_uuid(uuid)?;
        }

        Ok(partition)
    }
}
