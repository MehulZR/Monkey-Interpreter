#!/bin/sh

# Check if rust is already installed
if ! command -v rustup;
then
  # Install Rustup (compiler)
  echo "Installing Rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

  # Adding binaries to path
  source "$HOME/.cargo/env"
else
  echo "Rust is already installed"
fi

# Check if wasm-pack is already installed
if ! command -v wasm-pack;
then
  # Install wasm-pack
  echo "Installing wasm-pack..."
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y
else
	echo "wasm-pack is already installed"
fi

# Build wasm binary
echo "Building Monkey-Interpreter wasm binary..."
pnpm wasm:build

# Build static html for the react client
echo "Build NextJS..."
pnpm next build
