// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

/// A value range.
#[derive(Debug)]
pub struct Range {
    /// Minimum value.
    lower_bound: u64,
    /// Maximum value.
    upper_bound: u64,
}

impl Range {
    #[doc(hidden)]
    #[allow(dead_code)]
    /// Creates a new `Range`.
    pub(crate) fn new(lower_bound: u64, upper_bound: u64) -> Range {
        log::debug!(
            "Range::new created a new `Range` with lower_bound: {} and upper_bound: {}",
            lower_bound,
            upper_bound
        );

        Self {
            lower_bound,
            upper_bound,
        }
    }

    /// Returns the `Range`'s lower bound.
    pub fn lower_bound(&self) -> u64 {
        self.lower_bound
    }

    /// Returns the `Range`'s upper bound.
    pub fn upper_bound(&self) -> u64 {
        self.upper_bound
    }
}
