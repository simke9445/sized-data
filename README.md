# sized-data

Rust trait and derive macro for calculating struct sizes in Solana programs using the Anchor framework.

## Overview

`sized-data` provides compile-time size calculations for Solana program data structures. It's particularly useful when working with account data in Anchor programs where you need to know exact sizes for account initialization.

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
sized-data = "0.1.1"
```

Example usage:
```rust
use sized_data::SizedData;
use anchor_lang::prelude::*;

#[derive(SizedData)]
pub struct UserAccount {
    pub authority: Pubkey,
    pub counter: u64,
    pub data: [u8; 32],
}

// Initialize account with correct size
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = UserAccount::size_padded() // Uses padded size for Anchor
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

## Features

- Automatic size calculation for structs
- Built-in implementations for common Solana types (Pubkey, u64, u8, [u8; 32])
- Padded size calculation (adds 8 bytes for Anchor's discriminator)
- Zero-cost abstraction - all calculations done at compile time

## Size Implementations

| Type | Size (bytes) |
|------|-------------|
| u8 | 1 |
| u64 | 8 |
| Pubkey | 32 |
| [u8; 32] | 32 |

## Requirements

- Rust 1.83.0 or later
- anchor-lang 0.30.1 or later

## License

MIT License

For more details, visit [GitHub Repository](https://github.com/simke9445/sized-data)