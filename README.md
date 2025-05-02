# rsfdisk

----

[![Crates.io Version](https://img.shields.io/crates/v/rsfdisk?labelColor=%23222222&color=%23fdb42f)][1]
[![docs.rs](https://img.shields.io/docsrs/rsfdisk?labelColor=%23222222&color=%2322a884)][2]
![Crates.io MSRV](https://img.shields.io/crates/msrv/rsfdisk?labelColor=%23222222&color=%239c179e)
![Crates.io License](https://img.shields.io/crates/l/rsfdisk?labelColor=%23222222&color=%230d0887)

⚠️ WARNING: **This library is still in development, thus not yet suitable for
use in production.**

The `rsfdisk` library is a safe Rust wrapper around [`util-linux/libfdisk`][3].

rsfdisk can create and/or manipulate partition tables on block devices. It
understands `GPT`, `MBR`, `Sun`, `SGI`, and `BSD` partition tables.

## Usage

This crate requires `libfdisk` version `2.39.2` or later.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rsfdisk = "0.1.0"
```

Then install the system packages below before running `cargo build`:

- `util-linux`: to generate Rust bindings from `libfdisk`'s header files.
- `libclang`: to satisfy the [dependency][4] of [`bindgen`][5] on `libclang`.
- `pkg-config`: to detect system libraries.

Read the [installation instructions](#install-required-dependencies) below to
install the required dependencies on your system.

[Documentation (docs.rs)][2]

## Example

In this example we create a `GPT` partition table on `/dev/vda`, then divide it
into three partitions:

- a 16 GiB `root` partition to keep system files,
- and two 64 GiB data partitions.

```rust
use rsfdisk::fdisk::Fdisk;
use rsfdisk::partition_table::PartitionTableKind;
use rsfdisk::partition::Guid;
use rsfdisk::partition::Partition;
use rsfdisk::partition::PartitionKind;
use rsfdisk::partition::PartitionList;

fn main() -> rsfdisk::Result<()> {
    let mut disk = Fdisk::builder()
        // Operate on `/dev/vda`.
        .assign_device("/dev/vda")
        // Allow Fdisk to persist changes to disk.
        .enable_read_write()
        // Remove all existing partition tables, file systems, and RAID
        // signatures on the assigned device before writing a new partition
        // table.
        .wipe_device_metadata()
        .build()?;

    // Create a `GPT` partition table.
    disk.partition_table_create(PartitionTableKind::GPT)?;

    // Configure a 16 GiB System partition
    let partition_type = PartitionKind::builder()
       // Set the partition type identifier for a GUID/GPT partition table.
       .guid(Guid::LinuxRootx86_64)
       .build()?;

    let root = Partition::builder()
       .partition_type(partition_type)
       .name("System")
       //Assuming 512 bytes per sector, 33,554,432 sectors <=> 16 GiB.
       .size_in_sectors(33_554_432)
       .build()?;

    // Create the root partition.
    let _ = disk.partition_add(root)?;

    // Configure two 64 GiB data partitions.
    let mut data_partitions = PartitionList::new()?;

    // Assuming 512 bytes per sector, 68,719,476,736 sectors <=> 64 GiB.
    let size = 68_719_476_736;

    for i in 0..2 {
        let partition_type = PartitionKind::builder()
           .guid(Guid::LinuxData)
           .build()?;

        let name = format!("Data Part {}", i + 1);

        let partition = Partition::builder()
           .partition_type(partition_type)
           .name(name)
           .size_in_sectors(size)
           .build()?;

        data_partitions.push(partition)?;
    }

    // Create the data partitions.
    disk.partitions_append(data_partitions)?;

    // Write the new partition table on `/dev/vda`.
    disk.partition_table_write_to_disk()?;

    Ok(())
}
```

## Install required dependencies

### Alpine Linux

As `root`, issue the following command:

```console
apk add util-linux-dev clang-libclang pkgconfig
```

### NixOS

Install the packages in a temporary environment with:

```console
nix-shell -p util-linux.dev libclang.lib pkg-config
```

or permanently with:

```console
nix-env -iA nixos.util-linux.dev nixos.libclang.lib nixos.pkg-config
```

### Ubuntu

```console
sudo apt-get install libfdisk-dev libclang-dev pkg-config
```

## License

This project is licensed under either of the following:

- [Apache License, Version 2.0][6]
- [MIT License][7]

at your discretion.

Files in the [third-party/][8] and [web-snapshots/][9] directories are subject
to their own licenses and/or copyrights.

SPDX-License-Identifier: Apache-2.0 OR MIT

Copyright (c) 2023 Nick Piaddo

[1]: https://crates.io/crates/rsfdisk
[2]: https://docs.rs/rsfdisk
[3]: https://github.com/util-linux/util-linux/tree/master
[4]: https://rust-lang.github.io/rust-bindgen/requirements.html#clang
[5]: https://crates.io/crates/bindgen
[6]: https://www.apache.org/licenses/LICENSE-2.0
[7]: https://opensource.org/licenses/MIT
[8]: ./third-party/
[9]: ./web-snapshots/
