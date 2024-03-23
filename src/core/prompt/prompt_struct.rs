// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::errors::PromptError;
use crate::core::prompt::MenuItem;
use crate::core::prompt::PromptKind;
use crate::ffi_to_string_or_empty;
use crate::ffi_utils;

/// Interface for dialog-driven device partitioning, warning, and info messages.
#[derive(Debug)]
#[repr(transparent)]
pub struct Prompt {
    inner: *mut libfdisk::fdisk_ask,
}

impl Prompt {
    /// Returns the type of this `Prompt`.
    pub fn kind(&self) -> PromptKind {
        let code = unsafe { libfdisk::fdisk_ask_get_type(self.inner) };
        let kind = PromptKind::try_from(code as u32).unwrap();
        log::debug!("Prompt::kind value: {:?}", kind);

        kind
    }

    /// Returns the error number associated with a warning/error message.
    pub fn error_number(&self) -> i32 {
        let err_no = unsafe { libfdisk::fdisk_ask_print_get_errno(self.inner) };
        log::debug!("Prompt::error_number value: {:?}", err_no);

        err_no
    }

    /// Returns the content of a warning/error message.
    pub fn error_message(&self) -> Option<&str> {
        log::debug!("Prompt::error_message getting error message");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_ask_print_get_mesg(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("Prompt::error_message no error message. libfdisk::fdisk_ask_print_get_mesg returned a NULL pointer");

                None
            }
            msg_ptr => {
                let msg = ffi_utils::const_char_array_to_str_ref(msg_ptr).ok();
                log::debug!("Prompt::error_message got error message: {:?}", msg);

                msg
            }
        }
    }

    /// Returns the text of a question/request in this `Prompt`.
    pub fn query(&self) -> Option<&str> {
        log::debug!("Prompt::query getting prompt text");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_ask_get_query(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("Prompt::query got empty prompt text. libfdisk::fdisk_ask_get_query returned a NULL pointer");

                None
            }
            text_ptr => {
                let text = ffi_utils::const_char_array_to_str_ref(text_ptr).ok();
                log::debug!("Prompt::query got prompt text: {:?}", text);

                text
            }
        }
    }

    /// Returns the keyboard key code associated with the default menu item.
    pub fn menu_default_key(&self) -> i32 {
        let key = unsafe { libfdisk::fdisk_ask_menu_get_default(self.inner) };
        log::debug!(
            "Prompt::menu_default_key key code of default menu item: {:?}",
            key
        );

        key
    }

    /// Returns the total number of menu items.
    pub fn menu_count_items(&self) -> usize {
        let items = unsafe { libfdisk::fdisk_ask_menu_get_nitems(self.inner) };
        log::debug!(
            "Prompt::menu_count_items total number of menu items: {:?}",
            items
        );

        items
    }

    /// Returns the `nth` menu item.
    pub fn menu_nth_item(&self, nth: usize) -> Option<MenuItem> {
        log::debug!(
            "Prompt::menu_nth_item getting menu item at index: {:?}",
            nth
        );

        let mut name_ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        let mut desc_ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        let mut key_code = MaybeUninit::<libc::c_int>::zeroed();

        let result = unsafe {
            libfdisk::fdisk_ask_menu_get_item(
                self.inner,
                nth,
                key_code.as_mut_ptr(),
                name_ptr.as_mut_ptr(),
                desc_ptr.as_mut_ptr(),
            )
        };

        match result {
            0 => {
                let name_ptr = unsafe { name_ptr.assume_init() };
                let name = ffi_to_string_or_empty!(name_ptr);

                let desc_ptr = unsafe { desc_ptr.assume_init() };
                let description = ffi_to_string_or_empty!(desc_ptr);

                let key = unsafe { key_code.assume_init() };

                let menu_item = MenuItem::new(name, description, key);
                log::debug!(
                    "Prompt::menu_nth_item got menu item: {:?} at index: {:?}",
                    menu_item,
                    nth
                );

                Some(menu_item)
            }
            code if code > 0 => {
                log::debug!("Prompt::menu_nth_item index {:?} out of range", nth);

                None
            }
            code => {
                let err_msg = format!("failed to get menu item at index: {:?}", nth);
                log::debug!("Prompt::menu_nth_item {}. libfdisk::fdisk_ask_menu_get_item returned error code: {:?}", err_msg, code);

                None
            }
        }
    }

    /// Selects the menu item associated with the `key`.
    pub fn menu_item_select(&mut self, key: i32) -> Result<(), PromptError> {
        log::debug!(
            "Prompt::menu_item_select selecting item with key code: {:?}",
            key
        );

        let result = unsafe { libfdisk::fdisk_ask_menu_set_result(self.inner, key) };

        match result {
            0 => {
                log::debug!(
                    "Prompt::menu_item_select selected item with key code: {:?}",
                    key
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to select menu item with key code: {:?}", key);
                log::debug!("Prompt::menu_item_select {}. libfdisk::fdisk_ask_menu_set_result returned error code: {:?}", err_msg, code);

                Err(PromptError::Selection(err_msg))
            }
        }
    }

    /// Returns the key code of the keyboard key assigned to the currently selected item.
    pub fn menu_selected_item(&self) -> Option<i32> {
        log::debug!("Prompt::menu_selected_item getting key code value of selected key");

        let mut key_ptr = MaybeUninit::<libc::c_int>::zeroed();

        let result =
            unsafe { libfdisk::fdisk_ask_menu_get_result(self.inner, key_ptr.as_mut_ptr()) };

        match result {
            0 => {
                let key_code = unsafe { key_ptr.assume_init() };
                log::debug!(
                    "Prompt::menu_selected_item key code value of selected key: {:?}",
                    key_code
                );

                Some(key_code)
            }
            code => {
                log::debug!("Prompt::menu_selected_item failed to get selected key. libfdisk::fdisk_ask_menu_get_result returned error code: {:?}", code);

                None
            }
        }
    }

    #[doc(hidden)]
    /// Enables/Disables relative number notation.
    fn number_set_relative(ptr: *mut libfdisk::fdisk_ask, enable: bool) -> Result<(), PromptError> {
        let op = if enable { 1 } else { 0 };
        let op_str = if enable {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };

        let result = unsafe { libfdisk::fdisk_ask_number_set_relative(ptr, op) };

        match result {
            0 => {
                log::debug!(
                    "Prompt::number_set_relative {}d relative number notation",
                    op_str
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to {} relative number notation", op_str);
                log::debug!("Prompt::number_set_relative {}. libfdisk::fdisk_ask_number_set_relative returned error code: {:?}", err_msg, code);

                Err(PromptError::Config(err_msg))
            }
        }
    }

    /// Allows specifying numbers in relative notation when providing a numerical value.
    pub fn number_enable_relative(&mut self) -> Result<(), PromptError> {
        log::debug!("Prompt::number_enable_relative enabling relative number notation");

        Self::number_set_relative(self.inner, true)
    }

    /// Disallows specifying numbers in relative notation when providing a numerical value.
    pub fn number_disable_relative(&mut self) -> Result<(), PromptError> {
        log::debug!("Prompt::number_disable_relative disabling relative number notation");

        Self::number_set_relative(self.inner, false)
    }

    /// Sets the answer to a `Prompt` for a numerical value.
    pub fn number_set_answer(&mut self, value: u64) -> Result<(), PromptError> {
        log::debug!(
            "Prompt::number_set_answer answering prompt with value {:?}",
            value
        );

        let result = unsafe { libfdisk::fdisk_ask_number_set_result(self.inner, value) };

        match result {
            0 => {
                log::debug!(
                    "Prompt::number_set_answer answered prompt with value {:?}",
                    value
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to answer prompt with value: {:?}", value);
                log::debug!("Prompt::number_set_answer {}. libfdisk::fdisk_ask_number_set_result returned error code: {:?}", err_msg, code);

                Err(PromptError::Config(err_msg))
            }
        }
    }

    /// Returns the answer to the numerical value requested by the `Prompt`.
    pub fn number_answer(&self) -> u64 {
        let answer = unsafe { libfdisk::fdisk_ask_number_get_result(self.inner) };
        log::debug!("Prompt::number_answer value: {:?}", answer);

        answer
    }

    /// Returns the number of bytes per unit.
    pub fn number_bytes_per_unit(&self) -> u64 {
        let bytes = unsafe { libfdisk::fdisk_ask_number_get_unit(self.inner) };
        log::debug!("Prompt::number_bytes_per_unit value: {:?}", bytes);

        bytes
    }

    /// Returns the default value displayed as a possible answer to this `Prompt`.
    pub fn number_default(&self) -> u64 {
        let default = unsafe { libfdisk::fdisk_ask_number_get_default(self.inner) };
        log::debug!("Prompt::number_default default number: {:?}", default);

        default
    }

    /// Returns the value of the upper bound in the requested range.
    pub fn number_upper_bound(&self) -> u64 {
        let upper = unsafe { libfdisk::fdisk_ask_number_get_high(self.inner) };
        log::debug!("Prompt::number_upper_bound value: {:?}", upper);

        upper
    }

    /// Returns the value of the lower bound in the requested range.
    pub fn number_lower_bound(&self) -> u64 {
        let lower = unsafe { libfdisk::fdisk_ask_number_get_low(self.inner) };
        log::debug!("Prompt::number_lower_bound value: {:?}", lower);

        lower
    }

    /// Returns a string representation of the range in this `Prompt`.
    pub fn number_range(&self) -> Option<&str> {
        log::debug!("Prompt::number_range getting requested value range");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_ask_number_get_range(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("Prompt::number_range found no range. libfdisk::fdisk_ask_number_get_range returned a NULL pointer");

                None
            }
            range_ptr => {
                let range = ffi_utils::const_char_array_to_str_ref(range_ptr).ok();
                log::debug!("Prompt::number_range value: {:?}", range);

                range
            }
        }
    }

    /// Returns the reference point from which relative sizes are computed (i.e. when users specify
    /// values in relative notation `(+size)`).
    pub fn number_reference_point(&self) -> u64 {
        let reference = unsafe { libfdisk::fdisk_ask_number_get_base(self.inner) };
        log::debug!("Prompt::number_reference_point value: {:?}", reference);

        reference
    }

    /// Returns the answer to the string value requested by this `Prompt`.
    pub fn string_answer(&self) -> Option<&str> {
        log::debug!("Prompt::string_answer getting string answer");

        let mut ptr = MaybeUninit::<*mut libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_ask_string_get_result(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("Prompt::string_answer got no answer. libfdisk::fdisk_ask_string_get_result returned a NULL pointer");

                None
            }
            answer_ptr => {
                let answer = ffi_utils::const_char_array_to_str_ref(answer_ptr).ok();
                log::debug!("Prompt::string_answer got answer: {:?}", answer);

                answer
            }
        }
    }

    /// Sets the answer to a `Prompt` for a string value.
    pub fn string_set_answer<T>(&mut self, value: T) -> Result<(), PromptError>
    where
        T: AsRef<str>,
    {
        let value = value.as_ref();
        log::debug!(
            "Prompt::string_answer answering prompt with value: {:?}",
            value
        );

        let value_cstr = ffi_utils::as_ref_str_to_c_string(value)?;

        // Create a C-allocated copy to be managed by libfdisk.
        let mut copy = MaybeUninit::<*mut libc::c_char>::zeroed();
        unsafe {
            copy.write(libc::strndup(
                value_cstr.as_ptr(),
                value_cstr.as_bytes().len(),
            ));
        }

        let copy = unsafe { copy.assume_init() };

        if copy.is_null() {
            return Err(PromptError::Allocation(format!(
                "failed to create a C-allocated copy of {:?}",
                value
            )));
        }

        let result = unsafe { libfdisk::fdisk_ask_string_set_result(self.inner, copy) };

        match result {
            0 => {
                log::debug!(
                    "Prompt::string_answer answered prompt with value: {:?}",
                    value
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to answer prompt with value: {:?}", value);
                log::debug!("Prompt::string_answer {}. libfdisk::fdisk_ask_string_set_result returned error code: {:?}", err_msg, code);

                Err(PromptError::Config(err_msg))
            }
        }
    }

    /// Returns `true` when the answer to the `Prompt` was `yes`.
    pub fn yes_no_answer(&self) -> bool {
        let answer = unsafe { libfdisk::fdisk_ask_yesno_get_result(self.inner) };
        let answer_bool = answer == 1;
        log::debug!("Prompt::yes_no_answer value: {:?}", answer_bool);

        answer_bool
    }

    /// Sets the answer to this Yes/No `Prompt` to `"yes"` if `answer` is `true`.
    pub fn yes_no_set_answer(&mut self, answer: bool) -> Result<(), PromptError> {
        let answer_str = if answer {
            "yes".to_owned()
        } else {
            "no".to_owned()
        };
        let answer = if answer { 1 } else { 0 };
        log::debug!(
            "Prompt::yes_no_set_answer setting answer to yes/no question to: {:?}",
            answer_str
        );

        let result = unsafe { libfdisk::fdisk_ask_yesno_set_result(self.inner, answer) };

        match result {
            0 => {
                log::debug!("Prompt::yes_no_set_answer set answer to: {:?}", answer_str);

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to set answer to yes/no question to: {:?}",
                    answer_str
                );
                log::debug!("Prompt::yes_no_set_answer {}. libfdisk::fdisk_ask_yesno_set_result returned error code: {:?}", err_msg, code);

                Err(PromptError::Config(err_msg))
            }
        }
    }

    /// Returns `true` if this `Prompt` accepts negative numbers as input. In that case, the final
    /// answer is calculated down from the range's upper bound.
    pub fn accepts_negative_numbers(&self) -> bool {
        let accepts = unsafe { libfdisk::fdisk_ask_number_is_wrap_negative(self.inner) == 1 };
        log::debug!("Prompt::accepts_negative_numbers value: {:?}", accepts);

        accepts
    }

    /// Returns `true` if this `Prompt` matches any of the [`PromptKind`] variants.
    pub fn is_of_kind(&self, kind: PromptKind) -> bool {
        let state = unsafe { libfdisk::fdisk_ask_get_type(self.inner) as u32 == kind.into() };
        log::debug!(
            "Prompt::is_of_kind is `Prompt` of kind {:?}: {:?}",
            kind,
            state
        );

        state
    }

    /// Returns `true` when partitions should be addressed by letters instead of numbers (e.g the
    /// first partition in a `BSD disklabel` is `a`).
    pub fn requires_lettered_partitions(&self) -> bool {
        let required = unsafe { libfdisk::fdisk_ask_number_inchars(self.inner) == 1 };
        log::debug!("Prompt::requires_lettered_partitions value: {:?}", required);

        required
    }
}

impl AsRef<Prompt> for Prompt {
    #[inline]
    fn as_ref(&self) -> &Prompt {
        self
    }
}

impl Drop for Prompt {
    fn drop(&mut self) {
        log::debug!("Prompt::drop deallocating `Prompt` instance");

        unsafe { libfdisk::fdisk_unref_ask(self.inner) }
    }
}
