// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

/// A partition type's shortcut representation.
#[derive(Debug)]
pub struct Shortcut {
    alias: String,
    shortcut: String,
    type_string: String,
    alias_deprecated: bool,
}

impl Shortcut {
    #[doc(hidden)]
    #[allow(dead_code)]
    /// Creates a `Shortcut`.
    pub(crate) fn new(
        alias: String,
        shortcut: String,
        type_string: String,
        alias_deprecated: bool,
    ) -> Shortcut {
        let shortcut = Self {
            alias,
            shortcut,
            type_string,
            alias_deprecated,
        };
        log::debug! {"Shortcut::new created new `Shortcut` instance: {:?}", shortcut};

        shortcut
    }

    /// Returns a partition type's string alias.
    pub fn alias(&self) -> &str {
        &self.alias
    }

    /// Returns a partition type's shortcut representation.
    pub fn shortcut(&self) -> &str {
        &self.shortcut
    }

    /// Returns a partition type's string representation.
    pub fn type_string(&self) -> &str {
        &self.type_string
    }

    /// Returns `true` when this `Shortcut` has a deprecated alias value.
    pub fn has_alias_deprecated(&self) -> bool {
        self.alias_deprecated
    }
}
