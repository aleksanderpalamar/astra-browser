#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_ID="io.github.aleksanderpalamar.AstraBrowser"
SOURCES="$ROOT/data/icons/src"
THEME="$ROOT/data/icons/hicolor"

render() {
    local source="$1" size="$2"
    local target="$THEME/${size}x${size}/apps/$APP_ID.png"
    mkdir -p "$(dirname "$target")"
    rsvg-convert --width "$size" --height "$size" "$source" --output "$target"
}

for size in 16 22 24 32; do
    render "$SOURCES/astra-small.svg" "$size"
done
for size in 48 64 128 256 512; do
    render "$SOURCES/astra.svg" "$size"
done
