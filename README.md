# Anchor Vault Project

A complete Anchor-based Solana vault example, with a local development setup and test automation.

## Overview

This repository contains an Anchor program named `program_vault` and a JavaScript/TypeScript test setup for localnet development. The project is configured to run on a local Solana cluster and uses Anchor tooling to build, deploy, and test the vault functionality.

## Repository Layout

- `Anchor.toml` - Anchor configuration file for cluster, wallet, and program addresses.
- `Cargo.toml` - Workspace configuration for the Rust Anchor program.
- `package.json` - Node package manifest with Anchor and test dependencies.
- `programs/program-vault/` - Anchor program source and program-specific configuration.
- `tests/` - Anchor integration tests written in TypeScript.
- `migrations/` - Deployment scripts for Anchor.

## Prerequisites

Before you begin, make sure you have the following installed:

- `rust` with the Solana toolchain installed
- `solana-cli`
- `anchor-cli`
- `node` and `npm` or `yarn`
- `typescript`

If you use `yarn`, the commands below will work directly.

## Setup

### 1. Install dependencies

From the repository root:

```bash
cd /home/cyd3er/Solana-Learner/Vault/program-vault
yarn install
```

### 2. Configure Solana localnet

Start a local Solana validator in a separate terminal:

```bash
solana-test-validator
```

Ensure your local wallet keypair is available at:

```bash
~/.config/solana/id.json
```

If you need to create one:

```bash
solana-keygen new --outfile ~/.config/solana/id.json
```

### 3. Verify Anchor configuration

The Anchor configuration in `Anchor.toml` includes:

- `cluster = "Localnet"`
- `wallet = "~/.config/solana/id.json"
- `programs.localnet.program_vault` points to the local program ID used for deployment.

## Build and Deploy

### Build the Anchor program

```bash
anchor build
```

This compiles the Rust program in `programs/program-vault/` and generates the IDL and build artifacts.

### Deploy to localnet

```bash
anchor deploy
```

If deploy succeeds, Anchor updates the localnet program address and writes the deployed program data.

## Testing

This project uses TypeScript tests under `tests/`.

Run tests with:

```bash
yarn test
```

The provided `package.json` includes a `test` command that uses `ts-mocha` with a generous timeout:

```json
"test": "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

## Linting

Use Prettier to format JavaScript and TypeScript sources:

```bash
yarn lint
```

To automatically format files:

```bash
yarn lint:fix
```

## Notes

- The Anchor program ID is configured for localnet deployment and may be updated when the program is re-deployed.
- If you add or change instruction definitions in `programs/program-vault/src/`, rebuild and re-run Anchor deployment before running tests.
- The test suite assumes a local validator is running and Anchor is configured to connect to `Localnet`.

## Recommended Workflow

1. Start `solana-test-validator` in one terminal.
2. Run `anchor build`.
3. Run `anchor deploy`.
4. Run `yarn test`.
5. Use `yarn lint` or `yarn lint:fix` to maintain formatting.

## Helpful Commands Summary

```bash
yarn install
solana-test-validator
anchor build
anchor deploy
yarn test
yarn lint
yarn lint:fix
```

## Contact

For questions, inspect the Anchor program source in `programs/program-vault/src/` and the integration tests in `tests/program-vault.ts`.
