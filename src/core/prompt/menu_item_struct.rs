// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

/// A prompt menu item.
#[derive(Debug)]
pub struct MenuItem {
    name: String,
    description: String,
    key_code: i32,
}

impl MenuItem {
    #[doc(hidden)]
    #[allow(dead_code)]
    /// Creates a new `MenuItem`.
    pub(crate) fn new(name: String, description: String, key_code: i32) -> MenuItem {
        log::debug!("MenuItem::new creating a new `MenuItem` instance with name: {:?}, description: {:?}, key code: {:?}", name, description, key_code);

        Self {
            name,
            description,
            key_code,
        }
    }

    /// Returns the `MenuItem`'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the `MenuItem`'s description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the keyboard key code associated with this `MenuItem`.
    pub fn key_code(&self) -> i32 {
        self.key_code
    }
}
