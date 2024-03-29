// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use typed_builder::TypedBuilder;

// From standard library

// From this library
use crate::core::errors::PartitionKindBuilderError;
use crate::core::partition::Code;
use crate::core::partition::Guid;
use crate::core::partition::PartitionKind;

#[derive(Debug, TypedBuilder)]
#[builder(
    builder_type(
        name = PartitionKindBuilder,
        vis = "pub",
        doc ="Configure and instantiate a [`PartitionKind`].\n\nFor usage, see [`PartitionKindBuilder::build`]."),
    build_method(vis = "", name = __make)
)]
pub(crate) struct PartTypeBuilder {
    #[allow(dead_code)]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Set the partition type identifier (exclusively for `MBR` partition tables)."
        )
    )]
    code: Option<Code>,

    #[allow(dead_code)]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Set the partition type identifier (exclusively for `GUID` or `GPT` partition tables)."
        )
    )]
    guid: Option<Guid>,

    #[allow(dead_code)]
    #[builder(
        default,
        setter(
        transform = |type_num: u32, type_string: impl AsRef<str>| Some((type_num, type_string.as_ref().to_owned())),
        doc = "Set the identifier for an 'unknown' partition type (i.e. a partition type listed in neither by [`Code`] nor by [`Guid`]).\n\n
# Arguments\n\n
- `type_num`: type as a number\n
- `type_string`: type as a string"
    ))]
    unknown_kind: Option<(u32, String)>,

    #[allow(dead_code)]
    #[builder(
        default,
        setter(into, strip_option, doc = "Set the partition type name.")
    )]
    name: Option<String>,
}

#[allow(non_camel_case_types)]
impl<
        __code: ::typed_builder::Optional<Option<Code>>,
        __guid: ::typed_builder::Optional<Option<Guid>>,
        __unknown_kind: ::typed_builder::Optional<Option<(u32, String)>>,
        __name: ::typed_builder::Optional<Option<String>>,
    > PartitionKindBuilder<(__code, __guid, __unknown_kind, __name)>
{
    pub fn build(self) -> Result<PartitionKind, PartitionKindBuilderError> {
        let builder = self.__make();

        let mut partition_kind = match (builder.code, builder.guid, builder.unknown_kind) {
            (Some(code), None, None) => {
                let mut part_kind = PartitionKind::new()?;
                part_kind.set_code(code)?;

                Ok(part_kind)
            }
            (None, Some(guid), None) => {
                let mut part_kind = PartitionKind::new()?;
                part_kind.set_guid(guid)?;

                Ok(part_kind)
            }
            (None, None, Some((code, type_string))) => PartitionKind::new_unkown(code, type_string)
                .map_err(PartitionKindBuilderError::from),
            (None, None, None) => {
                let err_msg =
                    "you must call one of the following methods: `code`, `guid`, or `unknown_kind`"
                        .to_owned();
                log::debug!("PartitionKindBuilderError::build {}", err_msg);

                Err(PartitionKindBuilderError::Required(err_msg))
            }
            _otherwise => {
                let err_msg =
                    "methods `code`, `guid`, and `unknown_kind` can not be called at the same time"
                        .to_owned();
                log::debug!("PartitionKindBuilderError::build {}", err_msg);

                Err(PartitionKindBuilderError::MutuallyExclusive(err_msg))
            }
        }?;

        if let Some(name) = builder.name {
            partition_kind.set_name(name)?;
        }

        Ok(partition_kind)
    }
}
