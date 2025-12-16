#!/bin/sh
# qemu-watcher.sh - run QEMU through this wrapper and exit when a crash
# pattern is observed on QEMU's stdout/stderr (serial). Defaults to 'PANIC!'.
# Usage: qemu-watcher.sh [--pattern REGEX] -- <qemu-command> [args...]

set -u

PATTERN="PANIC!|DOUBLE FAULT"

# Parse args until --
while [ "$#" -gt 0 ]; do
    case "$1" in
        --pattern)
            shift
            if [ $# -eq 0 ]; then
                echo "qemu-watcher: missing pattern after --pattern" >&2
                exit 2
            fi
            PATTERN="$1"
            shift
            ;;
        --pattern=*)
            PATTERN="${1#--pattern=}"
            shift
            ;;
        --)
            shift
            break
            ;;
        *)
            # stop if we hit command (no -- provided)
            break
            ;;
    esac
done

if [ $# -eq 0 ]; then
    echo "qemu-watcher: no command provided. Usage: qemu-watcher.sh [--pattern REGEX] -- <qemu-cmd> [args...]" >&2
    exit 2
fi

CMD="$1"
shift

# create fifo for capturing output
TMPF="$(mktemp -u /tmp/qemu-watcher.XXXXXX)"
mkfifo "$TMPF"
trap 'rm -f "$TMPF"' EXIT INT TERM

# start the command with its stdout+stderr redirected into fifo
"$CMD" "$@" >"$TMPF" 2>&1 &
CHILD=$!

RC=0
# read the fifo and scan for pattern, printing lines as they come
while IFS= read -r LINE; do
    printf '%s\n' "$LINE"
    printf '%s\n' "$LINE" | grep -q -E "$PATTERN" 2>/dev/null
    if [ $? -eq 0 ]; then
        printf 'qemu-watcher: detected pattern "%s"; terminating child %d\n' "$PATTERN" "$CHILD" >&2
        kill "$CHILD" 2>/dev/null || true
        RC=1
        break
    fi
done < "$TMPF"

# give child a moment to exit
sleep 0.1
if kill -0 "$CHILD" 2>/dev/null; then
    kill "$CHILD" 2>/dev/null || true
fi
wait "$CHILD" 2>/dev/null || true

exit $RC
