# Ubuntu
apt install --fix-missing g++ pkg-config lld clang
apt install --fix-missing libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev

# Fedora

## dnf5
dnf5 install gcc-c++ lld clang
dnf5 install libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel

## dnf4
dnf install gcc-c++ lld clang
dnf install libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel

## rpm-ostree
rpm-ostree install gcc-c++ lld clang libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel

# cargo
cargo install cargo make