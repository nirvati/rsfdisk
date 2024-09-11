// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Module for working with partition tables.

// From dependency library

// From standard library

// From this library

pub use field_enum::Field;
pub use field_format_struct::FieldFormat;
pub use header_entry_content_struct::HeaderEntryContent;
pub use header_entry_enum::HeaderEntry;
pub use input_type_enum::InputType;
pub use max_col_width_enum::MaxColWidth;
pub use partition_table_dos_ext_trait::PartitionTableDOSExt;
pub use partition_table_gpt_ext_trait::PartitionTableGPTExt;
pub use partition_table_kind_enum::PartitionTableKind;
pub use partition_table_struct::PartitionTable;
pub use range_struct::Range;
pub use shortcut_struct::Shortcut;
pub use table_section_struct::TableSection;
pub use verification_status_enum::VerificationStatus;

mod field_enum;
mod field_format_struct;
mod header_entry_content_struct;
mod header_entry_enum;
mod input_type_enum;
mod max_col_width_enum;
mod partition_table_dos_ext_trait;
mod partition_table_gpt_ext_trait;
mod partition_table_kind_enum;
mod partition_table_struct;
mod range_struct;
mod shortcut_struct;
mod table_section_struct;
mod verification_status_enum;
