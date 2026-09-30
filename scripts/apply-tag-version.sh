#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TAG_PATTERN='^v([0-9]+\.[0-9]+\.[0-9]+)$'
MANIFEST="$ROOT/Cargo.toml"
LOCKFILE="$ROOT/Cargo.lock"
PKGBUILD="$ROOT/packaging/arch/PKGBUILD"

fail() {
    echo "$1" >&2
    exit 1
}

tag="${1:-}"
[[ "$tag" =~ $TAG_PATTERN ]] || fail "A tag '$tag' não segue o formato vMAJOR.MINOR.PATCH (ex.: v0.2.0)"
version="${BASH_REMATCH[1]}"

sed -i "0,/^version = \".*\"/s//version = \"$version\"/" "$MANIFEST"
sed -i "/^name = \"astra-browser\"$/{n;s/^version = \".*\"/version = \"$version\"/}" "$LOCKFILE"
sed -i "s/^pkgver=.*/pkgver=$version/" "$PKGBUILD"

grep -qx "version = \"$version\"" "$MANIFEST" || fail "Versão não aplicada em $MANIFEST"
grep -A1 -x 'name = "astra-browser"' "$LOCKFILE" | grep -qx "version = \"$version\"" ||
    fail "Versão não aplicada em $LOCKFILE"
grep -qx "pkgver=$version" "$PKGBUILD" || fail "Versão não aplicada em $PKGBUILD"

echo "$version"
