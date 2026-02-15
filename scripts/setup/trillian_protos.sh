#!/bin/bash
set -e
PROTO_DIR="src/shared/proto/trillian"
mkdir -p "$PROTO_DIR"
# Clone Trillian repo (shallow) to get protos
TMP_DIR=$(mktemp -d)
git clone --depth 1 https://github.com/google/trillian.git "$TMP_DIR"
# Copy all proto files recursively
cp -r "$TMP_DIR/trillian.proto" "$TMP_DIR/crypto" "$TMP_DIR/"*.proto "$PROTO_DIR/" 2>/dev/null || true
# Also copy necessary Google API protos? Trillian depends on google/api/annotations.proto
# We'll need to handle that. For simplicity, we'll use tonic-build with include paths later.
rm -rf "$TMP_DIR"
echo "Trillian protos copied to $PROTO_DIR"
