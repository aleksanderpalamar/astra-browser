#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_ID="io.github.aleksanderpalamar.AstraBrowser"
PREFIX="${PREFIX:-$HOME/.local}"
ICONS="$PREFIX/share/icons/hicolor"
BINARY="$PREFIX/bin/astra-browser"

cargo build --release --manifest-path "$ROOT/Cargo.toml"
install -Dm755 "$ROOT/target/release/astra-browser" "$BINARY"

mkdir -p "$PREFIX/share/applications"
sed "s|^Exec=astra-browser$|Exec=$BINARY|" "$ROOT/data/$APP_ID.desktop" \
    > "$PREFIX/share/applications/$APP_ID.desktop"

install -Dm644 "$ROOT/data/icons/src/astra.svg" "$ICONS/scalable/apps/$APP_ID.svg"
install -Dm644 "$ROOT/data/icons/src/astra-symbolic.svg" "$ICONS/symbolic/apps/$APP_ID-symbolic.svg"
for png in "$ROOT"/data/icons/hicolor/*/apps/"$APP_ID".png; do
    size="$(basename "$(dirname "$(dirname "$png")")")"
    install -Dm644 "$png" "$ICONS/$size/apps/$APP_ID.png"
done

if [[ -f "$ICONS/icon-theme.cache" ]] && command -v gtk4-update-icon-cache >/dev/null; then
    gtk4-update-icon-cache --quiet --force --ignore-theme-index "$ICONS"
fi

echo "Astra Browser instalado em $PREFIX"
