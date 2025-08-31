#!/usr/bin/env bash
set -euo pipefail

# Detect distro 
source /etc/os-release || true
ID_LIKE="${ID_LIKE:-}"
ID="${ID:-}"

echo "[*] Updating OS and installing dependencies..."

if [[ "${ID}" == "ubuntu" || "${ID_LIKE}" == *"debian"* ]]; then
  sudo apt update
  sudo apt upgrade -y
  sudo apt install -y build-essential pkg-config libssl-dev protobuf-compiler git curl
  sudo snap install rustup
  sudo apt  install cargo
elif [[ "${ID}" == "amzn" || "${ID_LIKE}" == *"rhel"* || "${ID_LIKE}" == *"fedora"* ]]; then
  sudo dnf -y update || true
  sudo dnf groupinstall -y "Development Tools"
  sudo dnf install -y gcc gcc-c++ make pkgconf-pkg-config openssl-devel protobuf-compiler git curl
else
  echo "[!] Unknown distro. Please install build tools, OpenSSL dev, Git, and protoc manually."
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
