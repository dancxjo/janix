#!/bin/bash
# debug_hang.sh - Run QEMU with monitor socket, capture state on Ctrl-C.
# Usage: ./scripts/debug_hang.sh <target_dir> -- <qemu-command> [args...]

set -u

if [ "$#" -lt 3 ]; then
    echo "Usage: $0 <target_dir> -- <qemu-command> [args...]" >&2
    exit 1
fi

TARGET_DIR="$1"
shift

if [ "$1" != "--" ]; then
    echo "Usage: $0 <target_dir> -- <qemu-command> [args...]" >&2
    exit 1
fi
shift

QEMU_PID=""
SOCK="qemu-monitor.sock"

cleanup() {
    if [ -n "${QEMU_PID:-}" ]; then
        kill "$QEMU_PID" 2>/dev/null || true
    fi
    rm -f "$SOCK"
}
trap cleanup EXIT TERM

handle_sigint() {
    echo ""
    echo "--- CAUGHT SIGINT: DUMPING STATE ---"

    if [ -S "$SOCK" ]; then
        echo "Dumping registers to qemu.log..."
        # Append marker
        echo "--- DEBUG SNAPSHOT ---" >> qemu.log

        # Use socat to send command and capture output
        if command -v socat >/dev/null; then
             echo "info registers" | socat - unix-connect:"$SOCK" >> qemu.log
             # info mem might be too verbose, skipping
        else
             echo "socat not found, cannot dump registers"
        fi
    else
        echo "Monitor socket not found!"
    fi

    kill "$QEMU_PID" 2>/dev/null || true
    wait "$QEMU_PID" 2>/dev/null || true

    echo "Analyzing log..."
    python3 scripts/analyze_crash.py qemu.log "$TARGET_DIR" || true
    python3 scripts/analyze_crash.py qemu.log boot || true

    exit 0
}
trap handle_sigint INT

rm -f qemu.log
rm -f "$SOCK"

echo "Starting QEMU... Press Ctrl-C to snapshot and analyze."
# Run QEMU command with monitor socket appended
"$@" -monitor unix:"$SOCK",server,nowait > qemu.log 2>&1 &
QEMU_PID=$!

wait "$QEMU_PID"
