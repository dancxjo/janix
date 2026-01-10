#!/bin/bash
set -e
ELF="$1"
if [ -z "$ELF" ]; then echo "Usage: $0 <elf>"; exit 1; fi

echo "Checking $ELF..."
readelf -h "$ELF" | grep "Type:"
TYPE=$(readelf -h "$ELF" | awk '/Type:/ {print $2}')

if [ "$TYPE" != "EXEC" ]; then
  echo "Error: ELF Type is $TYPE, expected EXEC"
  exit 1
fi

if readelf -l "$ELF" | grep -q "INTERP"; then
  echo "Error: ELF contains INTERP header (dynamic linker requested)"
  exit 1
fi

echo "✅ Verified $ELF is EXEC and non-dynamic."
