#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NUMBER='(0|[1-9][0-9]*)'
TAG_PATTERN="^v($NUMBER\.$NUMBER\.$NUMBER)(-([a-z]+(\.$NUMBER)?))?$"
MANIFEST="$ROOT/Cargo.toml"
LOCKFILE="$ROOT/Cargo.lock"
PKGBUILD="$ROOT/packaging/arch/PKGBUILD"

fail() {
    echo "$1" >&2
    exit 1
}

tag="${1:-}"
[[ "$tag" =~ $TAG_PATTERN ]] ||
    fail "A tag '$tag' não segue o formato vMAJOR.MINOR.PATCH ou vMAJOR.MINOR.PATCH-canal[.N] (ex.: v1.0.0, v1.0.0-beta.1)"
core="${BASH_REMATCH[1]}"
channel="${BASH_REMATCH[6]}"

semver="$core"
package_version="$core"
arch_version="$core"
if [[ -n "$channel" ]]; then
    semver="$core-$channel"
    package_version="$core~$channel"
    arch_version="$core$channel"
fi

sed -i "0,/^version = \".*\"/s//version = \"$semver\"/" "$MANIFEST"
sed -i "/^name = \"astra-browser\"$/{n;s/^version = \".*\"/version = \"$semver\"/}" "$LOCKFILE"
sed -i "s/^pkgver=.*/pkgver=$arch_version/" "$PKGBUILD"

grep -qx "version = \"$semver\"" "$MANIFEST" || fail "Versão não aplicada em $MANIFEST"
grep -A1 -x 'name = "astra-browser"' "$LOCKFILE" | grep -qx "version = \"$semver\"" ||
    fail "Versão não aplicada em $LOCKFILE"
grep -qx "pkgver=$arch_version" "$PKGBUILD" || fail "Versão não aplicada em $PKGBUILD"

echo "VERSION=$semver"
echo "PACKAGE_VERSION=$package_version"
echo "ARCH_VERSION=$arch_version"
