#!/usr/bin/env bash
set -euo pipefail

# === Config ==========================
REPO_DIR="$HOME/Key_distribution/rust_backend"
SERVICE_NAME="keydist.service"
BIN_DST_DIR="/opt/keydist"
BIN_DST="$BIN_DST_DIR/rust_backend"
CLIENT_PORT="50051"
REPL_PORT="50052"
# ================================================================

echo "[1/6] Enter repo & update code..."
cd "$REPO_DIR"
git fetch --all --prune || true
git pull --ff-only || true

echo "[2/6] Ensure cargo in PATH..."
if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
fi

echo "[3/6] Build release binary..."
cargo build --release --bin rust_backend

echo "[4/6] Install binary atomically with backup..."
sudo install -d "$BIN_DST_DIR"
TS="$(date +%Y%m%d-%H%M%S)"
if [ -x "$BIN_DST" ]; then
  echo "  - Backing up current binary to: ${BIN_DST}.bak-${TS}"
  sudo cp -f "$BIN_DST" "${BIN_DST}.bak-${TS}"
fi
# Copy new binary with correct mode; overwrite destination
sudo install -m 0755 "target/release/rust_backend" "$BIN_DST"

# If you use the dedicated service user, keep ownership (ignore if user absent)
if id keydist >/dev/null 2>&1; then
  sudo chown keydist:keydist "$BIN_DST"
fi

echo "[5/6] Restart service cleanly..."
# Refuse to restart if some OTHER process is already bound to our ports
if ss -ltn "sport = :$CLIENT_PORT" | grep -q LISTEN && \
   ! systemctl is-active --quiet "$SERVICE_NAME"; then
  echo "ERROR: Another process is listening on :$CLIENT_PORT but $SERVICE_NAME is not active."
  echo "       Resolve the conflict, then re-run deploy."
  exit 1
fi
if ss -ltn "sport = :$REPL_PORT" | grep -q LISTEN && \
   ! systemctl is-active --quiet "$SERVICE_NAME"; then
  echo "ERROR: Another process is listening on :$REPL_PORT but $SERVICE_NAME is not active."
  echo "       Resolve the conflict, then re-run deploy."
  exit 1
fi

# Reload units (harmless if unchanged) and restart
sudo systemctl daemon-reload
set +e
sudo systemctl restart "$SERVICE_NAME"
STATUS=$?
set -e

if [ $STATUS -ne 0 ]; then
  echo "✗ Restart failed. Attempting rollback..."
  if [ -f "${BIN_DST}.bak-${TS}" ]; then
    sudo cp -f "${BIN_DST}.bak-${TS}" "$BIN_DST"
    sudo systemctl restart "$SERVICE_NAME" || true
    echo "Rolled back to previous binary: ${BIN_DST}.bak-${TS}"
  else
    echo "No backup found to roll back."
  fi
  echo "Last logs:"
  journalctl -xeu "$SERVICE_NAME" -n 100 --no-pager || true
  exit 1
fi

echo "[6/6] Deployed ✓  Showing status & listeners..."
systemctl status "$SERVICE_NAME" -n 20 --no-pager || true
sudo ss -ltnp | grep -E "$CLIENT_PORT|$REPL_PORT" || true
