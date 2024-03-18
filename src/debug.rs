// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Activate debug message output.
//!
//! `rsfdisk` provides a facility to log debug messages through the
//! [log](https://crates.io/crates/log) lightweight logging *facade*.
//!
//! From the package's README, you need to provide a logger implementation compatible with the
//! *facade*:
//!
//! > In order to produce log output, executables have to use a logger implementation compatible with the facade.
//! > There are many available implementations to choose from, here are some options:
//! >
//! > * Simple minimal loggers:
//! >     * [`env_logger`](https://docs.rs/env_logger/*/env_logger/)
//! >     * [`colog`](https://docs.rs/colog/*/colog/)
//! >     * [`simple_logger`](https://docs.rs/simple_logger/*/simple_logger/)
//! >     * [`simplelog`](https://docs.rs/simplelog/*/simplelog/)
//! >     * [`pretty_env_logger`](https://docs.rs/pretty_env_logger/*/pretty_env_logger/)
//! >     * [`stderrlog`](https://docs.rs/stderrlog/*/stderrlog/)
//! >     * [`flexi_logger`](https://docs.rs/flexi_logger/*/flexi_logger/)
//! >     * [`call_logger`](https://docs.rs/call_logger/*/call_logger/)
//! >     * [`std-logger`](https://docs.rs/std-logger/*/std_logger/)
//! >     * [`structured-logger`](https://docs.rs/structured-logger/latest/structured_logger/)
//! > * Complex configurable frameworks:
//! >     * [`log4rs`](https://docs.rs/log4rs/*/log4rs/)
//! >     * [`logforth`](https://docs.rs/logforth/*/logforth/)
//! >     * [`fern`](https://docs.rs/fern/*/fern/)
//! > * Adaptors for other facilities:
//! >     * [`syslog`](https://docs.rs/syslog/*/syslog/)
//! >     * [`systemd-journal-logger`](https://docs.rs/systemd-journal-logger/*/systemd_journal_logger/)
//! >     * [`slog-stdlog`](https://docs.rs/slog-stdlog/*/slog_stdlog/)
//! >     * [`android_log`](https://docs.rs/android_log/*/android_log/)
//! >     * [`win_dbg_logger`](https://docs.rs/win_dbg_logger/*/win_dbg_logger/)
//! >     * [`db_logger`](https://docs.rs/db_logger/*/db_logger/)
//! >     * [`log-to-defmt`](https://docs.rs/log-to-defmt/*/log_to_defmt/)
//! >     * [`logcontrol-log`](https://docs.rs/logcontrol-log/*/logcontrol_log/)
//! > * For WebAssembly binaries:
//! >     * [`console_log`](https://docs.rs/console_log/*/console_log/)
//! > * For dynamic libraries:
//! >     * You may need to construct [an FFI-safe wrapper over `log`](https://github.com/rust-lang/log/issues/421) to initialize in your libraries.
//! > * Utilities:
//! >     * [`log_err`](https://docs.rs/log_err/*/log_err/)
//! >     * [`log-reload`](https://docs.rs/log-reload/*/log_reload/)
//! >     * [`alterable_logger`](https://docs.rs/alterable_logger/*/alterable_logger)
//! >
//! > Executables should choose a logger implementation and initialize it early in the
//! > runtime of the program. Logger implementations will typically include a
//! > function to do this. Any log messages generated before the logger is
//! > initialized will be ignored.
//! >
//! > The executable itself may use the `log` crate to log as well.
//!
//! Here is an example of debug message initialization using the
//! [`env_logger`](https://docs.rs/env_logger/*/env_logger/) crate, and `libfdisk`'s own debug
//! interface.
//!
//! ```ignore
//! static INIT: std::sync::Once = std::sync::Once::new();
//!
//! fn main() {
//!    // Initialize debug output
//!    INIT.call_once(|| {
//!        // rsfdisk debug messages
//!        env_logger::init();
//!        // libfdisk debug messages
//!        rsfdisk::debug::init_default_debug();
//!    });
//!
//!    // The rest of your program...
//!
//! }
//!
//! ```
//!
//! Assuming your executable is called `main` you can adjust the log-level of `libfdisk` and/or
//! `rsfdisk` by setting respectively the `LIBFDISK_DEBUG` and/or `RUST_LOG` environment variables.
//!
//! ```text
//! # libfdisk debug messages only
//! # (look to the `init_default_debug` function's documentation for an exhaustive list of options)
//! $ LIBFDISK_DEBUG="cxt,gpt" ./main
//! ```
//!
//! Example output:
//! ```text
//! libfdisk:     INIT: library debug mask: 0x0006
//! libfdisk:     INIT: library version: 2.39.2
//! libfdisk:      CXT: [0x7f9ed8000e30]: alloc
//! libfdisk:      CXT: [0x7f9ed8000e30]: assigning device /dev/vda
//! libfdisk:      CXT: [0x7f9ed8000e30]: *** resetting context
//! libfdisk:      CXT: [0x7f9ed8000e30]:   freeing firstsector
//! libfdisk:      CXT: [0x7f9ed8000e30]: /dev/vda: discovering topology...
//! libfdisk:      CXT: [0x7f9ed8000e30]: initialize libblkid prober
//! libfdisk:      CXT: [0x7f9ed8000e30]: result: log/phy sector size: 512/512
//! libfdisk:      CXT: [0x7f9ed8000e30]: result: fdisk/optimal/minimal io: 512/0/512
//! libfdisk:      CXT: [0x7f9ed8000e30]: /dev/vda: discovering geometry...
//! libfdisk:      CXT: [0x7f9ed8000e30]: total sectors: 102400 (ioctl=0)
//! libfdisk:      CXT: [0x7f9ed8000e30]: result: C/H/S: 6/255/63
//! libfdisk:      CXT: [0x7f9ed8000e30]: applying user device properties
//! libfdisk:      CXT: [0x7f9ed8000e30]: resetting alignment...
//! libfdisk:      CXT: [0x7f9ed8000e30]: alignment reset to: first LBA=2048, last LBA=102399, grain=1048576 [rc=0]
//! libfdisk:      CXT: [0x7f9ed8000e30]: initialize in-memory first sector buffer [sector_size=512]
//! libfdisk:      CXT: [0x7f9ed8000e30]: reading: offset=0, size=512
//! libfdisk:      CXT: [0x7f9ed8000e30]: new C/H/S: 6/255/63
//! libfdisk:      CXT: [0x7f9ed8000e30]: new log/phy sector size: 512/512
//! libfdisk:      CXT: [0x7f9ed8000e30]: zeroize in-memory first sector buffer
//! libfdisk:      CXT: [0x7f9ed8000e30]: reading: offset=0, size=512
//! libfdisk:      CXT: [0x7f9ed8000e30]: probing for gpt
//! libfdisk:      CXT: [0x7f9ed8000e30]: --> switching context to gpt!
//! ...snip...
//! ```
//!
//! ```text
//! # rsfdisk debug messages only
//! $ RUST_LOG=debug ./main
//! ```
//!
//! Example output:
//! ```text
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::builder creating a new `FdiskBuilder` instance
//! [DEBUG rsfdisk::fdisk::fdisk_builder_struct] FdiskBuilder::build building a new `Fdisk` instance
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::new creating a new `Fdisk` instance
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::new created a new `Fdisk` instance
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::assign_device_read_write assigning read-write device: /dev/vda
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::assign_device assigning read-write device: /dev/vda
//! [DEBUG rsfdisk::ffi_utils] as_ref_path_to_c_string converting `AsRef<Path>` to `CString`: /dev/vda
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::assign_device assigned read-write device: /dev/vda
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::disable_dialogs disabling partitioning prompts
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::display_dialogs disabled dialogs
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::enable_metadata_wipe marking device metadata for erasure
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::wipe_metadata enabled erasure of device metadata
//! [DEBUG rsfdisk::fdisk::fdisk_builder_struct] FdiskBuilder::build built a new `Fdisk` instance
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::partition_table_create creating GPT partition table
//! [DEBUG rsfdisk::ffi_utils] as_ref_str_to_c_string converting `&str` to `CString`: gpt
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::create_partition_table created partition table
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::partition_table_write_to_disk writing partition table to disk
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::partition_table_write_to_disk wrote partition table to disk
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::device_is_image_file value: true
//! [DEBUG rsfdisk::fdisk::fdisk_struct] Fdisk::drop deallocating `Fdisk` instance
//! ...snip...
//! ```
//!
//! Debugging modes can not be modified after calling [`init_default_debug`] or [`init_full_debug`]
//! once. The first function to get called sets the debug mode; a debug mode you can NOT change as
//! long as your program is running.

/// Activates library debugging messages. This function reads the `LIBFDISK_DEBUG` environment
/// variable to set the level of debug output.
///
/// It accepts the following values:
/// - `all`:      info about all subsystems
/// - `ask`:      fdisk dialogs
/// - `help`:     this help
/// - `cxt`:      library context (handler)
/// - `label`:    disk label utils
/// - `part`:     partition utils
/// - `parttype`: partition type utils
/// - `script`:   sfdisk-like scripts
/// - `tab`:      table utils
/// - `wipe`:     wipe area utils
/// - `item`:     disklabel items
/// - `gpt`:      GPT subsystems
///
/// # Examples
///
/// ```console
/// # You can set multiple values separated by commas
/// LIBFDISK_DEBUG="label,part,gpt"
/// ```
pub fn init_default_debug() {
    unsafe { libfdisk::fdisk_init_debug(0) }
}

/// Enables full debugging.
pub fn init_full_debug() {
    unsafe { libfdisk::fdisk_init_debug(0xffff) }
}
