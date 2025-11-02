#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPT_FILE="$SCRIPT_DIR/sample_shapes.shapdf"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

if ! command -v shapdf >/dev/null 2>&1; then
    echo "shapdf binary not found. Install it with 'cargo install shapdf'." >&2
    exit 1
fi

echo "Using example script: $SCRIPT_FILE"
echo "--- script begin ---"
cat "$SCRIPT_FILE"
echo "--- script end ---"

cd "$REPO_ROOT"

shapdf "$SCRIPT_FILE"

OUTPUT_FILE="${SCRIPT_FILE%.shapdf}.pdf"
echo "PDF generated at: $OUTPUT_FILE"
