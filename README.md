# Drishti

**A Fiscal Integrity Protocol for the Government of Nepal.**
Built on Solana using Anchor 0.30.x and the Token-2022 standard.

This document outlines the step-by-step instructions for getting the local development environment up and running smoothly. All team members must follow this guide to avoid version mismatch errors on Arch Linux.

## Requirements

Before starting, ensure your system has the following versions (or greater):
- Node.js `20.x LTS`
- Git

## Step-by-Step Environment Setup

### 1. Install System Dependencies
If you are on Arch Linux, use `yay` or `pacman` to install the Rust toolchain manager and Yarn:
```bash
yay -S rustup yarn
```

### 2. Configure Rust
We strictly use **Rust `1.83.0`** locally to bypass a known compilation bug in Anchor 0.30.x dependencies involving `proc-macro2`.
```bash
rustup install 1.83.0
rustup default 1.83.0
rustup override set 1.83.0
rustup component add clippy rustfmt
```

### 3. Install Solana CLI
Download and install the official Solana CLI binaries (ensure they are added to your `PATH`):
```bash
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

### 4. Install Anchor CLI (via AVM)
We use the Anchor Version Manager (AVM) to manage Anchor CLI versions.
```bash
# Ensure cargo is available in your PATH before running
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.30.1
avm use 0.30.1
```

### 5. Configure Localnet and Keypairs
Configure the Solana CLI targeting your local test validator with a fresh keypair:
```bash
mkdir -p ~/.config/solana
solana-keygen new --no-bip39-passphrase --outfile ~/.config/solana/id.json --force

solana config set --url localhost
solana config set --keypair ~/.config/solana/id.json

# Verify your configuration
solana config get
```

## Running the Project

Now that your dependencies are properly scaffolded, you can install the Node modules and verify the build works:

```bash
# Install node dependencies
yarn install

# Build the Anchor program (this compiles the BPF payload and establishes the IDL)
anchor build
```

If everything completes successfully, the `./target` directory will be populated with your compiled binary, and you are ready to write business logic!
