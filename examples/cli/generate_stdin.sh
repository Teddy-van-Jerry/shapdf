#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUTPUT_FILE="$SCRIPT_DIR/sample_shapes_inline.pdf"

if ! command -v shapdf >/dev/null 2>&1; then
    echo "shapdf binary not found. Install it with 'cargo install shapdf'." >&2
    exit 1
fi

SCRIPT_CONTENT=$(cat <<'EOS'
# Sample shapdf script demonstrating multiple pages and shapes
set default_page_size 210mm 210mm
set default_color rgb(0.2,0.2,0.2)

page default
line 10mm 10mm 190mm 10mm width=2mm color=#ff6600 cap=round
circle 40mm 150mm 12mm color=gray(0.3)
rectangle 100mm 110mm 50mm 25mm anchor=center angle=25deg color=rgb(0.2,0.6,0.9)

page letter
line 25mm 30mm 140mm 220mm width=1.2mm color=green
circle 120mm 180mm 8mm color=#aa00ff
rectangle 50mm 90mm 60mm 55mm anchor=north angle=330deg color=#444444
EOS
)

echo "Generating PDF from inline shapdf script via stdin"
echo "--- script begin ---"
printf '%s\n' "$SCRIPT_CONTENT"
echo "--- script end ---"

cd "$REPO_ROOT"

printf '%s\n' "$SCRIPT_CONTENT" | shapdf --output "$OUTPUT_FILE" -

echo "PDF generated at: $OUTPUT_FILE"
