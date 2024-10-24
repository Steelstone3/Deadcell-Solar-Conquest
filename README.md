# Deadcell Solar Conquest

Deadcell solar conquest is a space map control tactical rts-strategy game written in rust.

## Running Deadcell Solar Conquest

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
>
> install the packages
>
> "alsa-sys"
>
> "libudev-sys"

For apt

> sudo apt install librust-alsa-sys-dev librust-libudev-sys-dev

For dnf

> sudo dnf install rust-alsa-sys-devel rust-libudev-sys-devel

Or

> bash setup_dependencies.sh

To remove the additional dependencies run

> remove the packages
>
> "alsa-sys"
>
> "libudev-sys"

For apt

> sudo apt purge librust-alsa-sys-dev librust-libudev-sys-dev

For dnf

> sudo dnf remove rust-alsa-sys-devel rust-libudev-sys-devel

Or

> bash remove_dependencies.sh

Please use good op-sec habits and check the contents of both .sh files before running them as these will modify your system

## Credits

This game is made possible by a selection of talented asset creators who's work is published under open licenses making this game possible. Atributions are as follows:

- [pixel planet generator](<https://deep-fold.itch.io/pixel-planet-generator>) by Deep-Fold under the MIT License
- [space background generator](<https://deep-fold.itch.io/space-background-generator>) by Deep-Fold under the MIT License
- [void fleet pack 1](<https://foozlecc.itch.io/void-fleet-pack-1>) by Foozle under the CC0 1.0 Universal License
- [void fleet pack 2](<https://foozlecc.itch.io/void-fleet-pack-2>) by Foozle under the CC0 1.0 Universal License
- [void fleet pack 3](<https://foozlecc.itch.io/void-fleet-pack-3>) by Foozle under the CC0 1.0 Universal License
