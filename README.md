# pbuilder

## Reproducible builds of 🥵PArch

Build reproducible system images from platform manifests to build the PArch
platform for **P**i-style single-board computers (SBCs), based on the **Arch**
Linux operating system.

## Usage

Fetch or build for the SBC model

```sh
cargo run -- fetch <sbc_platform> [--overwrite]
```

Example:

```sh
cargo run -- fetch rpi2w --overwrite
```


```
### Build

```sh
cargo run -- build <sbc_platform> [--dryrun]
```

Example:

```sh
cargo run -- build rpi5 --dryrun
```

## Status

Early development. The interface and manifest format may change.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE.txt](LICENSE.txt).

Copyright 2026 Matt Turner https://mat.phd

This repository contains the public, general-purpose builder. Production recipes, infrastructure, signing materials, and internal processes are maintained separately.

