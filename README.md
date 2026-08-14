# Deadcell Solar Conquest

Deadcell solar conquest is a space map control tactical rts-strategy game written in rust.

## Running Deadcell Solar Conquest Server

> cd ~/Deadcell-Solar-Conquest
>
> cargo build
>
> cargo run server

## Running Deadcell Solar Conquest Client

> cd ~/Deadcell-Solar-Conquest
>
> cargo build
>
> cargo run

## Tests

> cd ~/Deadcell-Solar-Conquest
>
> cargo test

## Dependencies

Follow the steps for installing rustc runtime for your given operating system.

> <https://www.rust-lang.org/tools/install>

### Ubuntu

> apt install --fix-missing g++ pkg-config lld clang libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev

### Fedora

dnf5

> dnf5 install gcc-c++ lld clang libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel

dnf4

> dnf install gcc-c++ lld clang libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel

rpm-ostree

> rpm-ostree install clang libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel

### Cargo

cargo install cargo make

## Credits

This game is made possible by a selection of talented asset creators who's work is published under open licenses making this game possible. Atributions are as follows:

- [pixel planet generator](<https://deep-fold.itch.io/pixel-planet-generator>) by Deep-Fold under the MIT License
- [space background generator](<https://deep-fold.itch.io/space-background-generator>) by Deep-Fold under the MIT License
