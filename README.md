# pbuilder

## Reproducible builds of 🥵PArch

Build reproducible system images from platform manifests to build the PArch platform for **P**i-style single-board computers, based on the **Arch** Linux operating system.

## Usage

```sh
cargo run -- build <target> [--dryrun]
```

Example:

```sh
cargo run -- build rpi5 --dryrun
```

## Status

Early development. The interface and manifest format may change.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE.txt](LICENSE.txt).

Copyright 2026 Sustainable Inference.

This repository contains the public, general-purpose builder. Production recipes, infrastructure, signing materials, and internal processes are maintained separately.

