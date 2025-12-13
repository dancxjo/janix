#!/usr/bin/env bash
# Poll a QEMU monitor TCP socket and print `info registers` periodically.
# Usage: ./scripts/qemu-monitor-poll.sh [HOST] [PORT] [INTERVAL_SEC]

HOST=${1:-127.0.0.1}
PORT=${2:-4444}
INTERVAL=${3:-2}

echo "qemu-monitor-poll: connecting to ${HOST}:${PORT}, interval=${INTERVAL}s"

while true; do
  # Send the command and print the response. Use a short timeout so the poller
  # doesn't hang if QEMU isn't listening yet or the connection drops.
  if command -v nc >/dev/null 2>&1; then
    printf 'info registers\n' | nc -w 2 "${HOST}" "${PORT}" || printf '--- monitor unreachable (%s:%s) ---\n' "${HOST}" "${PORT}"
  elif command -v socat >/dev/null 2>&1; then
    printf 'info registers\n' | socat - TCP:"${HOST}":"${PORT}",connecttimeout=2 || printf '--- monitor unreachable (%s:%s) ---\n' "${HOST}" "${PORT}"
  else
    echo "Error: neither 'nc' nor 'socat' found in PATH" >&2
    exit 2
  fi

  # Separate samples for readability
  echo "--- sample end ---"
  sleep "${INTERVAL}"
done
