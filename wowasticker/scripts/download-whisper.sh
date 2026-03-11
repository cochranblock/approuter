#!/bin/bash
# Download Whisper-Tiny GGUF for wowasticker (Candle-compatible, lmz/candle-whisper).
# Usage: ./scripts/download-whisper.sh [output_dir]
# Output: model-tiny-q4k.gguf, config-tiny.json, tokenizer-tiny.json, melfilters.bytes

set -e
OUT="${1:-.}"
mkdir -p "$OUT"
cd "$OUT"

BASE="https://huggingface.co/lmz/candle-whisper/resolve/main"
CANDLE="https://raw.githubusercontent.com/huggingface/candle/main/candle-examples/examples/whisper"

for f in model-tiny-q4k.gguf config-tiny.json tokenizer-tiny.json; do
  if [ ! -f "$f" ]; then
    echo "Downloading $f..."
    curl -sL -o "$f" "$BASE/$f"
  else
    echo "Exists: $f"
  fi
done

if [ ! -f "melfilters.bytes" ]; then
  echo "Downloading melfilters.bytes..."
  curl -sL -o melfilters.bytes "$CANDLE/melfilters.bytes"
else
  echo "Exists: melfilters.bytes"
fi

echo "Done. Set WOWASTICKER_WHISPER_PATH to $(pwd)/model-tiny-q4k.gguf"
