# Grid Payments Escrow - Native Rust

A secure, milestone-based escrow program for Grid P2P Marketplace built with native Rust for Solana.

## Features

- ⚡ **Initialize Escrow**: Buyer deposits funds into a PDA-based escrow account
- 💸 **Release Funds**: Buyer can release funds to seller upon completion
- 🔄 **Cancel Escrow**: Buyer can cancel and get refunded before releasing
- 🔒 **Secure PDAs**: Uses program-derived addresses for secure fund storage
- 📦 **Native Rust**: Built without Anchor framework for maximum control

## Instructions

### 1. InitializeEscrow
Creates a new escrow and deposits funds from buyer.

**Accounts:**
- `[signer, writable]` Buyer account
- `[writable]` Escrow PDA account
- `[]` Seller account
- `[]` System program

### 2. ReleaseFunds
Releases escrowed funds to the seller.

**Accounts:**
- `[signer, writable]` Buyer account
- `[writable]` Escrow PDA account
- `[writable]` Seller account

### 3. CancelEscrow
Cancels escrow and refunds the buyer.

**Accounts:**
- `[signer, writable]` Buyer account
- `[writable]` Escrow PDA account

## Building

```bash
cargo build-sbf
```

## Testing

```bash
cargo test-sbf
```

## Deployment

```bash
solana program deploy target/deploy/escrow_native.so
```
