#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$HOME/opt/Key_distribution/rust_backend"
BINARY_PATH="$REPO_DIR/target/release/rust_backend"
PORT=50051

echo "[*] Pulling latest code..."
cd "$REPO_DIR"
git pull --ff-only || true

echo "[*] Building release binary..."
cargo build --release

echo "[*] Stopping any existing process on port $PORT..."
if command -v fuser >/dev/null 2>&1; then
  fuser -k "${PORT}/tcp" || true
else
  pkill -f "$BINARY_PATH" || true
fi

echo "[*] Starting new instance in background..."
#nohup "$BINARY_PATH" > "$REPO_DIR/server.log" 2>&1 &

echo "[*] Deployment complete. Tail logs with:"
echo "    tail -f $REPO_DIR/server.log"
