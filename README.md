# pbuilder

## Reproducible builds of PArch

Build reproducible system images from platform manifests to build the PArch
platform for **P**i-style single-board computers (SBCs), based on the **Arch**
Linux operating system.

## Usage

Fetch or build for the SBC model

### Fetch

```sh
cargo run -- fetch <sbc_platform> [--overwrite]
```

Example:

```sh
cargo run -- fetch rpi2w --overwrite
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

## TODO


# TODO list for PArch-builder


## Building From _Foundations_

_Foundations_ are versions of the Arch Linux kernel and user space that we start
with for our PArch operating ecosystem. We have to implement basic stuff since
this project is just starting out.

## First Deliverable: Automated PArch Prep For RPi 2W

Our reference board that balances performance, efficiency, and customizability
is the Raspberry Pi 2W. It has a quad-core 1.0GHz processor, meaning it can do
moderately demanding tasks _in parallel_, or, in other words, at the same time;
browse the web and run a simulation in R, for example.

It is limited in memory, but our hypothesis is that is a strength. The purpose
of PArch is to provide a user ecosystem that is efficient but not at the price
of usability. Modern computers with Windows or MacOS need RAM to run lots of
programs at once, under self-imposed constraints of _planned obsolescence_.

Sustainable Inference products are designed to provide _permanent value_,
or as permanent as anything could be, and intended as a polar opposite to
planned obsolescence.

### Fetching, Building, and Configuring The Base System

- Fetch a copy of Arch Linux that will work with the ARM processor architecture
on the Raspberry Pi 2W as a first full prototype process, with others to follow.
    - Each platform is specified in a {platform-code}.yml, where platform code
    is `rpi2w`, `rpi5`, or `opi3` for example, for the Raspberry Pi Zero 2W, the
    Raspberry Pi 5, or the Orange Pi 3, respectively.
    - Preserve research products like links to installation instructions, kept
    in the {platform-code}.yml spec. Example: [ArchLinux|ARM Raspberry Pi Zero 2 Installation
    Instructions](https://archlinuxarm.org/platforms/armv8/broadcom/raspberry-pi-zero-2)
- Prompt for keyboard setup. Incorporate terminal search for keyboard.
    - If it's US, prompt for
- Install and configure git:
    - Prompt user to provide their name and email to configure for
    inclusion with commits as follows:
        1. Prompt to accept configuration wizard
        1. Prompt to give name (can be nickname like "Matt Turner")
        1. Prompt to give email (use the one associated with your GitHub
        account; [GitHub Docs link](https://docs.github.com/en/account-and-profile/how-tos/email-preferences/setting-your-commit-email-address))
    - Change the default branch name to `main`:
        1. Prompt to accept configuration, noting that default will
           `master` otherwise.


### Installation To SD Card

> What does SD stand for?

TODO: Outline this to-do list!
