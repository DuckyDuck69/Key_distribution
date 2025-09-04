#!/usr/bin/env bash
set -euo pipefail

# Detect distro 
source /etc/os-release || true
ID_LIKE="${ID_LIKE:-}"
ID="${ID:-}"

echo "[*] Updating OS and installing dependencies..."

sudo apt update && sudo apt upgrade -y
sudo apt install -y build-essential pkg-config libssl-dev protobuf-compiler git curl
if ! command -v rustc >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

# Install Rust if missing
if ! command -v rustc >/dev/null 2>&1; then
  echo "[*] Installing Rust toolchain (rustup + stable)..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

echo "[*] Versions:"
rustc --version
cargo --version
protoc --version || echo "protoc not found!"
