#!/bin/bash
set -e

echo "[ghostview] Installing dependencies..."
yarn install || npm install
echo "[ghostview] Building frontend..."
yarn build || npm run build

echo "[ghostview] Building Tauri backend..."
cd src-tauri
cargo build --release
cd ..

BIN=src-tauri/target/release/ghostview-tauri
if [ -f "$BIN" ]; then
  echo "[ghostview] Copying binary to /usr/local/bin (sudo required)"
  sudo cp "$BIN" /usr/local/bin/ghostview
  echo "[ghostview] Installed! Run with: ghostview"
else
  echo "[ghostview] Build failed: binary not found."
  exit 1
fi
