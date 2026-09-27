#!/usr/bin/env bash
set -euo pipefail

APP_ID="io.github.aleksanderpalamar.AstraBrowser"
PREFIX="${PREFIX:-$HOME/.local}"
ICONS="$PREFIX/share/icons/hicolor"

rm -f "$PREFIX/bin/astra-browser"
rm -f "$PREFIX/share/applications/$APP_ID.desktop"
rm -f "$ICONS/scalable/apps/$APP_ID.svg"
rm -f "$ICONS/symbolic/apps/$APP_ID-symbolic.svg"
rm -f "$ICONS"/*/apps/"$APP_ID".png

if [[ -f "$ICONS/icon-theme.cache" ]] && command -v gtk4-update-icon-cache >/dev/null; then
    gtk4-update-icon-cache --quiet --force --ignore-theme-index "$ICONS"
fi

echo "Astra Browser removido de $PREFIX"
