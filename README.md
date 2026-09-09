# pbuilder

Build reproducible system images from platform manifests.

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

