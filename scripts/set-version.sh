#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "Usage: $0 <version>"
  echo "  e.g. $0 0.2.0"
  echo ""
  echo "Updates version in Cargo.toml, vscode/package.json, commits, and tags."
  exit 1
}

[[ $# -eq 1 ]] || usage

VERSION="$1"
TAG="v${VERSION}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Validate semver (basic check)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
  echo "Error: '$VERSION' is not a valid semver version" >&2
  exit 1
fi

echo "Setting version to $VERSION ..."

# 1. Cargo.toml
sed -i '' -E "s/^version = \"[^\"]+\"/version = \"${VERSION}\"/" "$ROOT/Cargo.toml"
echo "  Cargo.toml      -> $VERSION"

# 2. vscode/package.json + package-lock.json
(cd "$ROOT/vscode" && npm version "$VERSION" --no-git-tag-version --allow-same-version --quiet >/dev/null)
echo "  package.json     -> $VERSION"
echo "  package-lock.json -> $VERSION"

# 3. Cargo.lock (regenerate)
(cd "$ROOT" && cargo generate-lockfile --quiet 2>/dev/null || true)
echo "  Cargo.lock       -> synced"

# 4. Commit version bump
(cd "$ROOT" && git add Cargo.toml Cargo.lock vscode/package.json vscode/package-lock.json)
(cd "$ROOT" && git commit -m "chore: bump version to $VERSION")
echo "  commit           -> chore: bump version to $VERSION"

# 5. Git tag (delete existing if same version, then create)
if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "  git tag $TAG already exists, replacing"
  git tag -d "$TAG" >/dev/null
fi
git tag "$TAG"
echo "  git tag          -> $TAG"

echo ""
echo "Done. Push with:"
echo "  git push origin HEAD --tags"
