// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::errors::ConversionError;

/// The maximum column width of data printed on the terminal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaxColWidth {
    /// Column width.
    Length(u16),
    /// Column width expressed as a percentage of the terminal's width.
    Percentage(u16),
}

impl TryFrom<f64> for MaxColWidth {
    type Error = ConversionError;

    fn try_from(width: f64) -> Result<Self, Self::Error> {
        match width {
            _ if (0. ..1.).contains(&width) => {
                let percent = width * 100.;
                let percent = percent.ceil() as u16;

                Ok(Self::Percentage(percent))
            }
            _ if width >= 1. => {
                let len = width.ceil() as u16;

                Ok(Self::Length(len))
            }
            _ => {
                let err_msg = format!("a width can not have a negative value: {:?}", width);
                log::debug!("MaxColWidth::try_from {}.", err_msg);

                Err(ConversionError::MaxColWidth(err_msg))
            }
        }
    }
}
