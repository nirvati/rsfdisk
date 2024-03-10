// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Table of Contents
//! 1. [Description](#description)
//! 2. [API structure](#api-structure)
//! 3. [From `libfdisk` to `rsfdisk`](#from-libfdisk-to-rsfdisk)
//!     1. [Basic handlers and setting](#basic-handlers-and-setting)
//!         1. [Context](#context)
//!         2. [Ask](#ask)
//!         3. [Alignment](#alignment)
//!         4. [Script](#script)
//!     2. [Partitioning](#partitioning)
//!         1. [Label](#label)
//!         2. [Partition](#partition)
//!         3. [Table](#table)
//!         4. [Partition types](#partition-types)
//!         5. [Label item](#label-item)
//!         6. [Field](#field)
//!     3. [Label specific functions](#label-specific-functions)
//!         1. [Dos](#dos)
//!         2. [UEFI GPT](#uefi-gpt)
//!         3. [SUN](#sun)
//!         4. [SGI](#sgi)
//!         5. [BSD](#bsd)
//!     4. [Misc](#misc)
//!         1. [Iterator](#iterator)
//!         2. [Utils](#utils)
//!         3. [Library Initialization](#library-initialization)
//!         4. [Version functions](#version-functions)
//!
//! ## Description
//!
//! The `rsfdisk` library is a safe Rust wrapper around `util-linux/libfdisk`.
//!
//! Like `libfdisk`, `rsfdisk` is a library for the creation and manipulation of partition tables
//! on block devices. It understands `GPT`, `MBR`, `Sun`, `SGI`, and `BSD` partition tables. It
//! provides the tools necessary for dividing block devices into *partitions*, also known as logical
//! disks. All metadata about partitions are recorded in a *partition table*, usually written on
//! the first sector of a device.
//!
//! ## API structure
//!
//! `rsfdisk`'s API is roughly divided into two parts:
//! - `fdisk`: the main library module holding the `Fdisk` struct to create/edit/modify partition
//! tables,
//! - `core`: the module holding specialised objects used and/or returned by `Fdisk`.
//!
//! Finally, look to the `debug` module if you need diagnostics during development.
//!
//! ## From `libfdisk` to `rsfdisk`
//!
//! This section maps `libfdisk` functions to `rsfdisk` methods. It follows the same layout as
//! `libfdisk`’s documentation. You can use it as a reference to ease the transition from one API
//! to the other.
//!
//! ### Basic handlers and setting
//! #### Context
//! | `libfdisk`                              | `rsfdisk` |
//! | ------------------                      | --------- |
//! | [`struct fdisk_context`][1]             |           |
//! | [`fdisk_assign_device`][2]              |           |
//! | [`fdisk_assign_device_by_fd`][3]        |           |
//! | [`fdisk_deassign_device`][4]            |           |
//! | [`fdisk_reassign_device`][5]            |           |
//! | [`fdisk_device_is_used`][6]             |           |
//! | [`fdisk_enable_bootbits_protection`][7] |           |
//! | [`fdisk_enable_details`][8]             |           |
//! | [`fdisk_enable_listonly`][9]            |           |
//! | [`fdisk_enable_wipe`][10]               |           |
//! | [`fdisk_disable_dialogs`][11]           |           |
//! | [`fdisk_get_alignment_offset`][12]      |           |
//! | [`fdisk_get_collision`][13]             |           |
//! | [`fdisk_get_devfd`][14]                 |           |
//! | [`fdisk_get_devmodel`][15]              |           |
//! | [`fdisk_get_devname`][16]               |           |
//! | [`fdisk_get_devno`][17]                 |           |
//! | [`fdisk_get_disklabel_item`][18]        |           |
//! | [`fdisk_get_first_lba`][19]             |           |
//! | [`fdisk_get_geom_cylinders`][20]        |           |
//! | [`fdisk_get_geom_heads`][21]            |           |
//! | [`fdisk_get_geom_sectors`][22]          |           |
//! | [`fdisk_get_grain_size`][23]            |           |
//! | [`fdisk_get_last_lba`][24]              |           |
//! | [`fdisk_get_minimal_iosize`][25]        |           |
//! | [`fdisk_get_nsectors`][26]              |           |
//! | [`fdisk_get_optimal_iosize`][27]        |           |
//! | [`fdisk_get_parent`][28]                |           |
//! | [`fdisk_get_physector_size`][29]        |           |
//! | [`fdisk_get_sector_size`][30]           |           |
//! | [`fdisk_get_size_unit`][31]             |           |
//! | [`fdisk_get_unit`][32]                  |           |
//! | [`fdisk_get_units_per_sector`][33]      |           |
//! | [`fdisk_has_dialogs`][34]               |           |
//! | [`fdisk_has_label`][35]                 |           |
//! | [`fdisk_has_protected_bootbits`][36]    |           |
//! | [`fdisk_has_wipe`][37]                  |           |
//! | [`fdisk_is_details`][38]                |           |
//! | [`fdisk_is_labeltype`][39]              |           |
//! | [`fdisk_is_listonly`][40]               |           |
//! | [`fdisk_is_ptcollision`][41]            |           |
//! | [`fdisk_is_readonly`][42]               |           |
//! | [`fdisk_is_regfile`][43]                |           |
//! | [`fdisk_new_context`][44]               |           |
//! | [`fdisk_new_nested_context`][45]        |           |
//! | [`fdisk_ref_context`][46]               |           |
//! | [`fdisk_reread_changes`][47]            |           |
//! | [`fdisk_reread_partition_table`][48]    |           |
//! | [`fdisk_set_first_lba`][49]             |           |
//! | [`fdisk_set_last_lba`][50]              |           |
//! | [`fdisk_set_size_unit`][51]             |           |
//! | [`fdisk_set_unit`][52]                  |           |
//! | [`fdisk_unref_context`][53]             |           |
//! | [`fdisk_use_cylinders`][54]             |           |
//!
//! [1]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-context
//! [2]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-assign-device
//! [3]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-assign-device-by-fd
//! [4]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-deassign-device
//! [5]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-reassign-device
//! [6]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-device-is-used
//! [7]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-enable-bootbits-protection
//! [8]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-enable-details
//! [9]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-enable-listonly
//! [10]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-enable-wipe
//! [11]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-disable-dialogs
//! [12]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-alignment-offset
//! [13]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-collision
//! [14]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-devfd
//! [15]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-devmodel
//! [16]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-devname
//! [17]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-devno
//! [18]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-disklabel-item
//! [19]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-first-lba
//! [20]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-geom-cylinders
//! [21]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-geom-heads
//! [22]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-geom-sectors
//! [23]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-grain-size
//! [24]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-last-lba
//! [25]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-minimal-iosize
//! [26]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-nsectors
//! [27]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-optimal-iosize
//! [28]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-parent
//! [29]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-physector-size
//! [30]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-sector-size
//! [31]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-size-unit
//! [32]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-unit
//! [33]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-get-units-per-sector
//! [34]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-has-dialogs
//! [35]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-has-label
//! [36]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-has-protected-bootbits
//! [37]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-has-wipe
//! [38]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-is-details
//! [39]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-is-labeltype
//! [40]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-is-listonly
//! [41]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-is-ptcollision
//! [42]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-is-readonly
//! [43]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-is-regfile
//! [44]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-new-context
//! [45]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-new-nested-context
//! [46]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-ref-context
//! [47]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-reread-changes
//! [48]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-reread-partition-table
//! [49]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-set-first-lba
//! [50]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-set-last-lba
//! [51]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-set-size-unit
//! [52]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-set-unit
//! [53]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-unref-context
//! [54]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Context.html#fdisk-use-cylinders
//!
//! #### Ask
//!
//! | `libfdisk`                                | `rsfdisk` |
//! | ------------------                        | --------- |
//! | [`struct fdisk_ask`][55]                  |           |
//! | [`enum   fdisk_asktype`][56]              |           |
//! | [`fdisk_info`][57]                        |           |
//! | [`fdisk_warn`][58]                        |           |
//! | [`fdisk_warnx`][59]                       |           |
//! | [`fdisk_set_ask`][60]                     |           |
//! | [`fdisk_is_ask`][61]                      |           |
//! | [`fdisk_ask_get_query`][62]               |           |
//! | [`fdisk_ask_get_type`][63]                |           |
//! | [`fdisk_ask_menu_get_default`][64]        |           |
//! | [`fdisk_ask_menu_get_item`][65]           |           |
//! | [`fdisk_ask_menu_get_nitems`][66]         |           |
//! | [`fdisk_ask_menu_get_result`][67]         |           |
//! | [`fdisk_ask_menu_set_result`][68]         |           |
//! | [`fdisk_ask_number`][69]                  |           |
//! | [`fdisk_ask_number_get_base`][70]         |           |
//! | [`fdisk_ask_number_get_default`][71]      |           |
//! | [`fdisk_ask_number_get_high`][72]         |           |
//! | [`fdisk_ask_number_get_low`][73]          |           |
//! | [`fdisk_ask_number_get_range`][74]        |           |
//! | [`fdisk_ask_number_get_result`][75]       |           |
//! | [`fdisk_ask_number_get_unit`][76]         |           |
//! | [`fdisk_ask_number_inchars`][77]          |           |
//! | [`fdisk_ask_number_is_wrap_negative`][78] |           |
//! | [`fdisk_ask_number_set_relative`][79]     |           |
//! | [`fdisk_ask_number_set_result`][80]       |           |
//! | [`fdisk_ask_partnum`][81]                 |           |
//! | [`fdisk_ask_print_get_errno`][82]         |           |
//! | [`fdisk_ask_print_get_mesg`][83]          |           |
//! | [`fdisk_ask_string`][84]                  |           |
//! | [`fdisk_ask_string_get_result`][85]       |           |
//! | [`fdisk_ask_string_set_result`][86]       |           |
//! | [`fdisk_ask_yesno`][87]                   |           |
//! | [`fdisk_ask_yesno_get_result`][88]        |           |
//! | [`fdisk_ask_yesno_set_result`][89]        |           |
//! | [`fdisk_ref_ask`][90]                     |           |
//! | [`fdisk_unref_ask`][91]                   |           |
//!
//! [55]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask
//! [56]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-asktype
//! [57]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-info
//! [58]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-warn
//! [59]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-warnx
//! [60]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-set-ask
//! [61]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-is-ask
//! [62]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-get-query
//! [63]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-get-type
//! [64]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-menu-get-default
//! [65]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-menu-get-item
//! [66]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-menu-get-nitems
//! [67]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-menu-get-result
//! [68]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-menu-set-result
//! [69]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number
//! [70]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-get-base
//! [71]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-get-default
//! [72]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-get-high
//! [73]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-get-low
//! [74]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-get-range
//! [75]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-get-result
//! [76]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-get-unit
//! [77]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-inchars
//! [78]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-is-wrap-negative
//! [79]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-set-relative
//! [80]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-number-set-result
//! [81]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-partnum
//! [82]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-print-get-errno
//! [83]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-print-get-mesg
//! [84]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-string
//! [85]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-string-get-result
//! [86]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-string-set-result
//! [87]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-yesno
//! [88]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-yesno-get-result
//! [89]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ask-yesno-set-result
//! [90]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-ref-ask
//! [91]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Ask.html#fdisk-unref-ask
//!
//! #### Alignment
//!
//! | `libfdisk`                               | `rsfdisk` |
//! | ------------------                       | --------- |
//! | [`typedef fdisk_sector_t`][92]           |           |
//! | [`fdisk_align_lba`][93]                  |           |
//! | [`fdisk_align_lba_in_range`][94]         |           |
//! | [`fdisk_has_user_device_properties`][95] |           |
//! | [`fdisk_lba_is_phy_aligned`][96]         |           |
//! | [`fdisk_override_geometry`][97]          |           |
//! | [`fdisk_reset_alignment`][98]            |           |
//! | [`fdisk_reset_device_properties`][99]    |           |
//! | [`fdisk_save_user_geometry`][100]        |           |
//! | [`fdisk_save_user_grain`][101]           |           |
//! | [`fdisk_save_user_sector_size`][102]     |           |
//!
//! [92]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-sector-t
//! [93]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-align-lba
//! [94]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-align-lba-in-range
//! [95]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-has-user-device-properties
//! [96]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-lba-is-phy-aligned
//! [97]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-override-geometry
//! [98]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-reset-alignment
//! [99]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-reset-device-properties
//! [100]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-save-user-geometry
//! [101]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-save-user-grain
//! [102]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Alignment.html#fdisk-save-user-sector-size
//!
//! #### Script
//!
//! | `libfdisk`                            | `rsfdisk` |
//! | ------------------                    | --------- |
//! | [`struct fdisk_script`][103]          |           |
//! | [`fdisk_set_script`][104]             |           |
//! | [`fdisk_get_script`][105]             |           |
//! | [`fdisk_apply_script`][106]           |           |
//! | [`fdisk_apply_script_headers`][107]   |           |
//! | [`fdisk_new_script`][108]             |           |
//! | [`fdisk_new_script_from_file`][109]   |           |
//! | [`fdisk_ref_script`][110]             |           |
//! | [`fdisk_script_enable_json`][111]     |           |
//! | [`fdisk_script_get_header`][112]      |           |
//! | [`fdisk_script_get_nlines`][113]      |           |
//! | [`fdisk_script_set_table`][114]       |           |
//! | [`fdisk_script_get_table`][115]       |           |
//! | [`fdisk_script_has_force_label`][116] |           |
//! | [`fdisk_script_read_context`][117]    |           |
//! | [`fdisk_script_read_file`][118]       |           |
//! | [`fdisk_script_read_line`][119]       |           |
//! | [`fdisk_script_set_header`][120]      |           |
//! | [`fdisk_script_set_fgets`][121]       |           |
//! | [`fdisk_script_write_file`][122]      |           |
//! | [`fdisk_script_set_userdata`][123]    |           |
//! | [`fdisk_script_get_userdata`][124]    |           |
//! | [`fdisk_unref_script`][125]           |           |
//!
//! [103]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script
//! [104]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-set-script
//! [105]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-get-script
//! [106]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-apply-script
//! [107]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-apply-script-headers
//! [108]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-new-script
//! [109]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-new-script-from-file
//! [110]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-ref-script
//! [111]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-enable-json
//! [112]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-get-header
//! [113]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-get-nlines
//! [114]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-set-table
//! [115]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-get-table
//! [116]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-has-force-label
//! [117]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-read-context
//! [118]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-read-file
//! [119]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-read-line
//! [120]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-set-header
//! [121]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-set-fgets
//! [122]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-write-file
//! [123]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-set-userdata
//! [124]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-script-get-userdata
//! [125]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Script.html#fdisk-unref-script
//!
//! ### Partitioning
//! #### Label
//!
//! | `libfdisk`                                    | `rsfdisk` |
//! | ------------------                            | --------- |
//! | [`struct fdisk_label`][126]                   |           |
//! | [`enum   fdisk_labeltype`][127]               |           |
//! | [`fdisk_create_disklabel`][128]               |           |
//! | [`fdisk_list_disklabel`][129]                 |           |
//! | [`fdisk_locate_disklabel`][130]               |           |
//! | [`fdisk_reorder_partitions`][131]             |           |
//! | [`fdisk_set_disklabel_id`][132]               |           |
//! | [`fdisk_set_disklabel_id_from_string`][133]   |           |
//! | [`fdisk_set_partition_type`][134]             |           |
//! | [`fdisk_toggle_partition_flag`][135]          |           |
//! | [`fdisk_verify_disklabel`][136]               |           |
//! | [`fdisk_write_disklabel`][137]                |           |
//! | [`fdisk_get_disklabel_id`][138]               |           |
//! | [`fdisk_get_label`][139]                      |           |
//! | [`fdisk_get_nlabels`][140]                    |           |
//! | [`fdisk_next_label`][141]                     |           |
//! | [`fdisk_get_npartitions`][142]                |           |
//! | [`fdisk_is_label`][143]()                     |           |
//! | [`fdisk_label_advparse_parttype`][144]        |           |
//! | [`fdisk_label_get_field`][145]                |           |
//! | [`fdisk_label_get_field_by_name`][146]        |           |
//! | [`fdisk_label_get_fields_ids`][147]           |           |
//! | [`fdisk_label_get_fields_ids_all`][148]       |           |
//! | [`fdisk_label_get_geomrange_cylinders`][149]  |           |
//! | [`fdisk_label_get_geomrange_heads`][150]      |           |
//! | [`fdisk_label_get_geomrange_sectors`][151]    |           |
//! | [`fdisk_label_get_name`][152]                 |           |
//! | [`fdisk_label_get_nparttypes`][153]           |           |
//! | [`fdisk_label_get_parttype`][154]             |           |
//! | [`fdisk_label_get_parttype_from_code`][155]   |           |
//! | [`fdisk_label_get_parttype_from_string`][156] |           |
//! | [`fdisk_label_get_parttype_shortcut`][157]    |           |
//! | [`fdisk_label_get_type`][158]                 |           |
//! | [`fdisk_label_has_code_parttypes`][159]       |           |
//! | [`fdisk_label_has_parttypes_shortcuts`][160]  |           |
//! | [`fdisk_label_is_changed`][161]               |           |
//! | [`fdisk_label_is_disabled`][162]              |           |
//! | [`fdisk_label_parse_parttype`][163]           |           |
//! | [`fdisk_label_require_geometry`][164]         |           |
//! | [`fdisk_label_set_changed`][165]              |           |
//! | [`fdisk_label_set_disabled`][166]             |           |
//!
//! [126]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label
//! [127]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-labeltype
//! [128]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-create-disklabel
//! [129]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-list-disklabel
//! [130]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-locate-disklabel
//! [131]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-reorder-partitions
//! [132]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-set-disklabel-id
//! [133]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-set-disklabel-id-from-string
//! [134]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-set-partition-type
//! [135]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-toggle-partition-flag
//! [136]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-verify-disklabel
//! [137]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-write-disklabel
//! [138]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-get-disklabel-id
//! [139]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-get-label
//! [140]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-get-nlabels
//! [141]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-next-label
//! [142]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-get-npartitions
//! [143]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-is-label
//! [144]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-advparse-parttype
//! [145]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-field
//! [146]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-field-by-name
//! [147]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-fields-ids
//! [148]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-fields-ids-all
//! [149]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-geomrange-cylinders
//! [150]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-geomrange-heads
//! [151]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-geomrange-sectors
//! [152]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-name
//! [153]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-nparttypes
//! [154]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-parttype
//! [155]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-parttype-from-code
//! [156]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-parttype-from-string
//! [157]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-parttype-shortcut
//! [158]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-get-type
//! [159]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-has-code-parttypes
//! [160]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-has-parttypes-shortcuts
//! [161]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-is-changed
//! [162]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-is-disabled
//! [163]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-parse-parttype
//! [164]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-require-geometry
//! [165]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-set-changed
//! [166]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Label.html#fdisk-label-set-disabled
//!
//! #### Partition
//!
//! | `libfdisk`                                     | `rsfdisk` |
//! | ------------------                             | --------- |
//! | [`struct fdisk_partition`][167]                |           |
//! | [`fdisk_add_partition`][168]                   |           |
//! | [`fdisk_delete_all_partitions`][169]           |           |
//! | [`fdisk_delete_partition`][170]                |           |
//! | [`fdisk_get_partition`][171]                   |           |
//! | [`fdisk_is_partition_used`][172]               |           |
//! | [`fdisk_set_partition`][173]                   |           |
//! | [`fdisk_wipe_partition`][174]                  |           |
//! | [`fdisk_new_partition`][175]                   |           |
//! | [`fdisk_partition_cmp_partno`][176]            |           |
//! | [`fdisk_partition_cmp_start`][177]             |           |
//! | [`fdisk_partition_end_follow_default`][178]    |           |
//! | [`fdisk_partition_end_is_default`][179]        |           |
//! | [`fdisk_partition_get_attrs`][180]             |           |
//! | [`fdisk_partition_get_end`][181]               |           |
//! | [`fdisk_partition_get_name`][182]              |           |
//! | [`fdisk_partition_get_parent`][183]            |           |
//! | [`fdisk_partition_get_partno`][184]            |           |
//! | [`fdisk_partition_get_size`][185]              |           |
//! | [`fdisk_partition_get_start`][186]             |           |
//! | [`fdisk_partition_get_type`][187]              |           |
//! | [`fdisk_partition_get_uuid`][188]              |           |
//! | [`fdisk_partition_has_end`][189]               |           |
//! | [`fdisk_partition_has_partno`][190]            |           |
//! | [`fdisk_partition_has_size`][191]              |           |
//! | [`fdisk_partition_has_start`][192]             |           |
//! | [`fdisk_partition_has_wipe`][193]              |           |
//! | [`fdisk_partition_is_bootable`][194]           |           |
//! | [`fdisk_partition_is_container`][195]          |           |
//! | [`fdisk_partition_is_freespace`][196]          |           |
//! | [`fdisk_partition_is_nested`][197]             |           |
//! | [`fdisk_partition_is_used`][198]               |           |
//! | [`fdisk_partition_is_wholedisk`][199]          |           |
//! | [`fdisk_partition_next_partno`][200]           |           |
//! | [`fdisk_partition_partno_follow_default`][201] |           |
//! | [`fdisk_partition_set_attrs`][202]             |           |
//! | [`fdisk_partition_set_name`][203]              |           |
//! | [`fdisk_partition_set_partno`][204]            |           |
//! | [`fdisk_partition_set_size`][205]              |           |
//! | [`fdisk_partition_set_start`][206]             |           |
//! | [`fdisk_partition_set_type`][207]              |           |
//! | [`fdisk_partition_set_uuid`][208]              |           |
//! | [`fdisk_partition_size_explicit`][209]         |           |
//! | [`fdisk_partition_start_follow_default`][210]  |           |
//! | [`fdisk_partition_start_is_default`][211]      |           |
//! | [`fdisk_partition_to_string`][212]             |           |
//! | [`fdisk_partition_unset_partno`][213]          |           |
//! | [`fdisk_partition_unset_size`][214]            |           |
//! | [`fdisk_partition_unset_start`][215]           |           |
//! | [`fdisk_ref_partition`][216]                   |           |
//! | [`fdisk_reset_partition`][217]                 |           |
//! | [`fdisk_unref_partition`][218]                 |           |
//!
//! [167]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition
//! [168]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-add-partition
//! [169]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-delete-all-partitions
//! [170]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-delete-partition
//! [171]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-get-partition
//! [172]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-is-partition-used
//! [173]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-set-partition
//! [174]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-wipe-partition
//! [175]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-new-partition
//! [176]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-cmp-partno
//! [177]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-cmp-start
//! [178]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-end-follow-default
//! [179]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-end-is-default
//! [180]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-get-attrs
//! [181]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-get-end
//! [182]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-get-name
//! [183]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-get-parent
//! [184]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-get-partno
//! [185]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-get-size
//! [186]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-get-start
//! [187]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-get-type
//! [188]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-get-uuid
//! [189]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-has-end
//! [190]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-has-partno
//! [191]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-has-size
//! [192]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-has-start
//! [193]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-has-wipe
//! [194]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-is-bootable
//! [195]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-is-container
//! [196]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-is-freespace
//! [197]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-is-nested
//! [198]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-is-used
//! [199]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-is-wholedisk
//! [200]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-next-partno
//! [201]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-partno-follow-default
//! [202]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-set-attrs
//! [203]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-set-name
//! [204]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-set-partno
//! [205]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-set-size
//! [206]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-set-start
//! [207]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-set-type
//! [208]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-set-uuid
//! [209]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-size-explicit
//! [210]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-start-follow-default
//! [211]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-start-is-default
//! [212]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-to-string
//! [213]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-unset-partno
//! [214]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-unset-size
//! [215]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-partition-unset-start
//! [216]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-ref-partition
//! [217]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-reset-partition
//! [218]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition.html#fdisk-unref-partition
//!
//! #### Table
//!
//! | `libfdisk`                                   | `rsfdisk` |
//! | ------------------                           | --------- |
//! | [`struct fdisk_table`][219]                  |           |
//! | [`fdisk_get_freespaces`][220]                |           |
//! | [`fdisk_get_partitions`][221]                |           |
//! | [`fdisk_apply_table`][222]                   |           |
//! | [`fdisk_new_table`][223]                     |           |
//! | [`fdisk_ref_table`][224]                     |           |
//! | [`fdisk_reset_table`][225]                   |           |
//! | [`fdisk_table_add_partition`][226]           |           |
//! | [`fdisk_table_get_nents`][227]               |           |
//! | [`fdisk_table_get_partition`][228]           |           |
//! | [`fdisk_table_get_partition_by_partno`][229] |           |
//! | [`fdisk_table_is_empty`][230]                |           |
//! | [`fdisk_table_next_partition`][231]          |           |
//! | [`fdisk_table_remove_partition`][232]        |           |
//! | [`fdisk_table_sort_partitions`][233]         |           |
//! | [`fdisk_table_wrong_order`][234]             |           |
//! | [`fdisk_unref_table`][235]                   |           |
//!
//! [219]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table
//! [220]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-get-freespaces
//! [221]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-get-partitions
//! [222]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-apply-table
//! [223]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-new-table
//! [224]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-ref-table
//! [225]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-reset-table
//! [226]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table-add-partition
//! [227]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table-get-nents
//! [228]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table-get-partition
//! [229]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table-get-partition-by-partno
//! [230]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table-is-empty
//! [231]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table-next-partition
//! [232]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table-remove-partition
//! [233]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table-sort-partitions
//! [234]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-table-wrong-order
//! [235]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Table.html#fdisk-unref-table
//!
//! #### Partition types
//!
//! | `libfdisk`                                  | `rsfdisk` |
//! | ------------------                          | --------- |
//! | [`struct fdisk_parttype`][236]              |           |
//! | [`enum   fdisk_parttype_parser_flags`][237] |           |
//! | [`fdisk_copy_parttype`][238]                |           |
//! | [`fdisk_new_parttype`][239]                 |           |
//! | [`fdisk_new_unknown_parttype`][240]         |           |
//! | [`fdisk_parttype_get_code`][241]            |           |
//! | [`fdisk_parttype_get_name`][242]            |           |
//! | [`fdisk_parttype_get_string`][243]          |           |
//! | [`fdisk_parttype_is_unknown`][244]          |           |
//! | [`fdisk_parttype_set_code`][245]            |           |
//! | [`fdisk_parttype_set_name`][246]            |           |
//! | [`fdisk_parttype_set_typestr`][247]         |           |
//! | [`fdisk_ref_parttype`][248]                 |           |
//! | [`fdisk_unref_parttype`][249]               |           |
//!
//! [236]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-parttype
//! [237]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-parttype-parser-flags
//! [238]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-copy-parttype
//! [239]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-new-parttype
//! [240]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-new-unknown-parttype
//! [241]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-parttype-get-code
//! [242]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-parttype-get-name
//! [243]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-parttype-get-string
//! [244]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-parttype-is-unknown
//! [245]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-parttype-set-code
//! [246]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-parttype-set-name
//! [247]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-parttype-set-typestr
//! [248]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-ref-parttype
//! [249]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Partition-types.html#fdisk-unref-parttype
//!
//! #### Label item
//!
//! | `libfdisk`                               | `rsfdisk` |
//! | ------------------                       | --------- |
//! | [`struct fdisk_labelitem`][250]          |           |
//! | [`enum   fdisk_labelitem_bsd`][251]      |           |
//! | [`enum   fdisk_labelitem_gen`][252]      |           |
//! | [`enum   fdisk_labelitem_gpt`][253]      |           |
//! | [`enum   fdisk_labelitem_sgi`][254]      |           |
//! | [`enum   fdisk_labelitem_sun`][255]      |           |
//! | [`fdisk_new_labelitem`][256]             |           |
//! | [`fdisk_ref_labelitem`][257]             |           |
//! | [`fdisk_reset_labelitem`][258]           |           |
//! | [`fdisk_unref_labelitem`][259]           |           |
//! | [`fdisk_labelitem_get_name`][260]        |           |
//! | [`fdisk_labelitem_get_id`][261]          |           |
//! | [`fdisk_labelitem_get_data_u64`][262]    |           |
//! | [`fdisk_labelitem_get_data_string`][263] |           |
//! | [`fdisk_labelitem_is_string`][264]       |           |
//! | [`fdisk_labelitem_is_number`][265]       |           |
//!
//! [250]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem
//! [251]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-bsd
//! [252]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-gen
//! [253]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-gpt
//! [254]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-sgi
//! [255]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-sun
//! [256]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-new-labelitem
//! [257]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-ref-labelitem
//! [258]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-reset-labelitem
//! [259]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-unref-labelitem
//! [260]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-get-name
//! [261]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-get-id
//! [262]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-get-data-u64
//! [263]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-get-data-string
//! [264]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-is-string
//! [265]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Labelitem.html#fdisk-labelitem-is-number
//!
//! #### Field
//!
//! | `libfdisk`                      | `rsfdisk` |
//! | ------------------              | --------- |
//! | [`struct fdisk_field`][266]     |           |
//! | [`enum   fdisk_fieldtype`][267] |           |
//! | [`fdisk_field_get_id`][268]     |           |
//! | [`fdisk_field_get_name`][269]   |           |
//! | [`fdisk_field_get_width`][270]  |           |
//! | [`fdisk_field_is_number`][271]  |           |
//!
//! [266]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Field.html#fdisk-field
//! [267]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Field.html#fdisk-fieldtype
//! [268]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Field.html#fdisk-field-get-id
//! [269]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Field.html#fdisk-field-get-name
//! [270]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Field.html#fdisk-field-get-width
//! [271]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Field.html#fdisk-field-is-number
//!
//! ### Label specific functions
//! #### DOS
//!
//! | `libfdisk`                           | `rsfdisk` |
//! | ------------------                   | --------- |
//! | [`DOS_FLAG_ACTIVE`][272]             |           |
//! | [`fdisk_dos_enable_compatible`][273] |           |
//! | [`fdisk_dos_is_compatible`][274]     |           |
//! | [`fdisk_dos_move_begin`][275]        |           |
//! | [`fdisk_dos_fix_chs`][276]           |           |
//!
//! [272]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-DOS.html#DOS-FLAG-ACTIVE:CAPS
//! [273]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-DOS.html#fdisk-dos-enable-compatible
//! [274]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-DOS.html#fdisk-dos-is-compatible
//! [275]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-DOS.html#fdisk-dos-move-begin
//! [276]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-DOS.html#fdisk-dos-fix-chs
//!
//! #### UEFI GPT
//!
//! | `libfdisk`                             | `rsfdisk` |
//! | ------------------                     | --------- |
//! | [`GPT_FLAG_REQUIRED`][278]             |           |
//! | [`GPT_FLAG_NOBLOCK`][279]              |           |
//! | [`GPT_FLAG_LEGACYBOOT`][280]           |           |
//! | [`GPT_FLAG_GUIDSPECIFIC`][281]         |           |
//! | [`fdisk_gpt_is_hybrid`][282]           |           |
//! | [`fdisk_gpt_get_partition_attrs`][283] |           |
//! | [`fdisk_gpt_set_partition_attrs`][284] |           |
//! | [`fdisk_gpt_set_npartitions`][285]     |           |
//! | [`fdisk_gpt_disable_relocation`][286]  |           |
//! | [`fdisk_gpt_enable_minimize`][287]     |           |
//!
//! [278]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#GPT-FLAG-REQUIRED:CAPS
//! [279]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#GPT-FLAG-NOBLOCK:CAPS
//! [280]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#GPT-FLAG-LEGACYBOOT:CAPS
//! [281]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#GPT-FLAG-GUIDSPECIFIC:CAPS
//! [282]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#fdisk-gpt-is-hybrid
//! [283]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#fdisk-gpt-get-partition-attrs
//! [284]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#fdisk-gpt-set-partition-attrs
//! [285]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#fdisk-gpt-set-npartitions
//! [286]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#fdisk-gpt-disable-relocation
//! [287]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-UEFI-GPT.html#fdisk-gpt-enable-minimize
//!
//! #### SUN
//!
//! | `libfdisk`                       | `rsfdisk` |
//! | ------------------               | --------- |
//! | [`fdisk_sun_set_alt_cyl`][288]   |           |
//! | [`fdisk_sun_set_ilfact`][289]    |           |
//! | [`fdisk_sun_set_pcylcount`][290] |           |
//! | [`fdisk_sun_set_rspeed`][291]    |           |
//! | [`fdisk_sun_set_xcyl`][292]      |           |
//!
//! [288]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-SUN.html#fdisk-sun-set-alt-cyl
//! [289]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-SUN.html#fdisk-sun-set-ilfact
//! [290]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-SUN.html#fdisk-sun-set-pcylcount
//! [291]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-SUN.html#fdisk-sun-set-rspeed
//! [292]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-SUN.html#fdisk-sun-set-xcyl
//!
//! #### SGI
//!
//! | `libfdisk`                      | `rsfdisk` |
//! | ------------------              | --------- |
//! | [`SGI_FLAG_BOOT`][293]          |           |
//! | [`SGI_FLAG_SWAP`][294]          |           |
//! | [`fdisk_sgi_create_info`][295]  |           |
//! | [`fdisk_sgi_set_bootfile`][296] |           |
//!
//! [293]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-SGI.html#SGI-FLAG-BOOT:CAPS
//! [294]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-SGI.html#SGI-FLAG-SWAP:CAPS
//! [295]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-SGI.html#fdisk-sgi-create-info
//! [296]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-SGI.html#fdisk-sgi-set-bootfile
//!
//! #### BSD
//!
//! | `libfdisk`                         | `rsfdisk` |
//! | ------------------                 | --------- |
//! | [`fdisk_bsd_edit_disklabel`][297]  |           |
//! | [`fdisk_bsd_link_partition`][298]  |           |
//! | [`fdisk_bsd_write_bootstrap`][299] |           |
//!
//! [297]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-BSD.html#fdisk-bsd-edit-disklabel
//! [298]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-BSD.html#fdisk-bsd-link-partition
//! [299]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-BSD.html#fdisk-bsd-write-bootstrap
//!
//! ### Misc
//! #### Iterator
//!
//! | `libfdisk`                        | `rsfdisk` |
//! | ------------------                | --------- |
//! | [`struct fdisk_iter`][300]        |           |
//! | [`fdisk_free_iter`][301]          |           |
//! | [`fdisk_iter_get_direction`][302] |           |
//! | [`fdisk_new_iter`][303]           |           |
//! | [`fdisk_reset_iter`][304]         |           |
//!
//! [300]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Iterator.html#fdisk-iter
//! [301]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Iterator.html#fdisk-free-iter
//! [302]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Iterator.html#fdisk-iter-get-direction
//! [303]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Iterator.html#fdisk-new-iter
//! [304]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Iterator.html#fdisk-reset-iter
//!
//! #### Utils
//!
//! | `libfdisk`              | `rsfdisk` |
//! | ------------------      | --------- |
//! | [`fdisk_partname`][305] |           |
//!
//! [305]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Utils.html#fdisk-partname
//!
//! #### Library initialization
//!
//! | `libfdisk`                | `rsfdisk` |
//! | ------------------        | --------- |
//! | [`fdisk_init_debug`][306] |           |
//!
//! [306]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Library-initialization.html#fdisk-init-debug
//!
//! #### Version functions
//!
//! | `libfdisk`                          | `rsfdisk` |
//! | ------------------                  | --------- |
//! | [`LIBFDISK_MAJOR_VERSION`][307]     |           |
//! | [`LIBFDISK_MINOR_VERSION`][308]     |           |
//! | [`LIBFDISK_PATCH_VERSION`][309]     |           |
//! | [`LIBFDISK_VERSION`][310]           |           |
//! | [`fdisk_parse_version_string`][311] |           |
//! | [`fdisk_get_library_version`][312]  |           |
//! | [`fdisk_get_library_features`][313] |           |
//!
//! [307]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Version-functions.html#LIBFDISK-MAJOR-VERSION:CAPS
//! [308]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Version-functions.html#LIBFDISK-MINOR-VERSION:CAPS
//! [309]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Version-functions.html#LIBFDISK-PATCH-VERSION:CAPS
//! [310]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Version-functions.html#LIBFDISK-VERSION:CAPS
//! [311]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Version-functions.html#fdisk-parse-version-string
//! [312]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Version-functions.html#fdisk-get-library-version
//! [313]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libfdisk-docs/libfdisk-Version-functions.html#fdisk-get-library-features
