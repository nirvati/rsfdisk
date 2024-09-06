// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Runtime errors.

// From dependency library

// From standard library

// From this library
pub use conversion_error_enum::ConversionError;
pub use gen_iterator_error_enum::GenIteratorError;
pub use header_entry_content_error_enum::HeaderEntryContentError;
pub use parser_error_enum::ParserError;
pub use partition_builder_error_enum::PartitionBuilderError;
pub use partition_error_enum::PartitionError;
pub use partition_iter_error_enum::PartitionIterError;
pub use partition_kind_builder_error_enum::PartitionKindBuilderError;
pub use partition_kind_error_enum::PartitionKindError;
pub use partition_list_error_enum::PartitionListError;
pub use partition_table_error_enum::PartitionTableError;
pub use prompt_error_enum::PromptError;
pub use script_error_enum::ScriptError;

mod conversion_error_enum;
mod gen_iterator_error_enum;
mod header_entry_content_error_enum;
mod parser_error_enum;
mod partition_builder_error_enum;
mod partition_error_enum;
mod partition_iter_error_enum;
mod partition_kind_builder_error_enum;
mod partition_kind_error_enum;
mod partition_list_error_enum;
mod partition_table_error_enum;
mod prompt_error_enum;
mod script_error_enum;
