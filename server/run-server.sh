#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
BIN="$SCRIPT_DIR/target/debug/deadcell-solar-conquest-server"

if [ ! -x "$BIN" ]; then
    echo "Server binary not found: $BIN"
    echo "Build it first with: cargo build"
    exit 1
fi

# If we are already running inside a terminal, replace the current shell process with
# the server binary. That is the only reliable way for the dedicated console to close
# when the application exits instead of returning to a shell prompt.
if [ -t 0 ] && [ -t 1 ]; then
    exec "$BIN"
fi

# Fallback for desktop launchers: try common terminal emulators and exec the
# server binary inside them so the emulator/window exits when the server exits.
# Use a shell wrapper that `exec`s the server to reliably replace the shell.
if command -v konsole >/dev/null 2>&1; then
    exec konsole -e /bin/sh -c "exec \"$BIN\""
fi

if command -v gnome-terminal >/dev/null 2>&1; then
    exec gnome-terminal -- /bin/sh -c "exec \"$BIN\""
fi

if command -v xfce4-terminal >/dev/null 2>&1; then
    exec xfce4-terminal -e /bin/sh -c "exec \"$BIN\""
fi

if command -v x-terminal-emulator >/dev/null 2>&1; then
    exec x-terminal-emulator -e /bin/sh -c "exec \"$BIN\""
fi

if command -v alacritty >/dev/null 2>&1; then
    exec alacritty -e /bin/sh -c "exec \"$BIN\""
fi

if command -v tilix >/dev/null 2>&1; then
    exec tilix -e /bin/sh -c "exec \"$BIN\""
fi

if command -v mate-terminal >/dev/null 2>&1; then
    exec mate-terminal -e /bin/sh -c "exec \"$BIN\""
fi

if command -v lxterminal >/dev/null 2>&1; then
    exec lxterminal -e /bin/sh -c "exec \"$BIN\""
fi

if command -v rxvt >/dev/null 2>&1; then
    exec rxvt -e /bin/sh -c "exec \"$BIN\""
fi

if command -v urxvt >/dev/null 2>&1; then
    exec urxvt -e /bin/sh -c "exec \"$BIN\""
fi

if command -v xterm >/dev/null 2>&1; then
    exec xterm -e /bin/sh -c "exec \"$BIN\""
fi

echo "No supported terminal emulator found. Please launch the binary from a terminal manually."
exit 1
