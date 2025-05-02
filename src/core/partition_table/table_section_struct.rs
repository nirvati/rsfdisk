// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

#[cfg_attr(doc,
         cfg_attr(all(),
        doc = ::embed_doc_image::embed_image!( "fig-01", "third-party/vendor/wikipedia/GUID_Partition_Table_Scheme.svg"),
        ))]
/// Metadata about a region of a partition table.
///
/// For example, a primary GPT partition table has three sections:
/// - a Protective MBR,
/// - a Partition Table Header,
/// - and a Partition Entry Array,
///   as illustrated on the diagram below.
///
/// ![Diagram illustrating the layout of the GUID Partition Table (GPT) scheme. Each logical
/// block (LBA) is 512 bytes in size. LBA addresses that are negative indicate position from
/// the end of the volume, with −1 being the last addressable block.][fig-01]
///
/// Source: <cite>The original uploader was Kbolino at [English
/// Wikipedia.](https://commons.wikimedia.org/wiki/File:GUID_Partition_Table_Scheme.svg), [CC
/// BY-SA
/// 2.5](https://creativecommons.org/licenses/by-sa/2.5), via Wikimedia Commons</cite>
///
#[derive(Debug)]
pub struct TableSection {
    name: String,
    starting_offset: u64,
    size: usize,
}

impl TableSection {
    #[allow(dead_code)]
    pub(crate) fn new(name: String, starting_offset: u64, size: usize) -> TableSection {
        log::debug!(
            "TableSection::new created a new `TableSection` instance with name: {}, starting offset: {}, size: {}",
            name,
            starting_offset,
            size
        );

        Self {
            name,
            starting_offset,
            size,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // FIXME offset in bytes or sectors?
    pub fn starting_offset(&self) -> u64 {
        self.starting_offset
    }

    // FIXME size in bytes or sectors?
    pub fn size(&self) -> usize {
        self.size
    }
}
