# Ubuntu
sudo apt install --fix-missing g++ pkg-config lld clang
doas apt install --fix-missing g++ pkg-config lld clang
sudo apt install --fix-missing libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev
doas apt install --fix-missing libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev

# Fedora

## dnf5
sudo dnf5 install gcc-c++ lld clang
doas dnf5 install gcc-c++ lld clang
sudo dnf5 install libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel
doas dnf5 install libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel

## dnf4
sudo dnf install gcc-c++ lld clang
doas dnf install gcc-c++ lld clang
sudo dnf install libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel
doas dnf install libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel