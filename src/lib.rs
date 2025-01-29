//! Sized data trait implementation for Solana programs using Anchor framework
//! 
//! This crate provides a trait and derive macro for calculating struct sizes at compile time,
//! which is particularly useful when working with Solana account initialization.
//!
//! # Example
//!
//! ```rust
//! use sized_data::SizedData;
//! use anchor_lang::prelude::*;
//!
//! #[derive(SizedData)]
//! pub struct UserAccount {
//!     pub authority: Pubkey,
//!     pub counter: u64,
//! }
//!
//! #[derive(Accounts)]
//! pub struct Initialize<'info> {
//!     #[account(
//!         init,
//!         payer = user,
//!         space = UserAccount::size_padded()
//!     )]
//!     pub user_account: Account<'info, UserAccount>,
//!     #[account(mut)]
//!     pub user: Signer<'info>,
//!     pub system_program: Program<'info, System>,
//! }
//! ```

use anchor_lang::prelude::Pubkey;

pub use sized_data_derive::*;

/// A trait for types that can calculate their size at compile time.
///
/// Used primarily with Anchor accounts to determine required space allocation.
/// The trait provides two methods:
/// - `size()`: Returns the raw size of the type
/// - `size_padded()`: Returns the size including Anchor's 8-byte discriminator
///
/// # Examples
///
/// ```rust
/// use sized_data::SizedData;
/// use anchor_lang::prelude::*;
///
/// #[derive(SizedData)]
/// pub struct UserAccount {
///     pub authority: Pubkey,  // 32 bytes
///     pub counter: u64,       // 8 bytes
/// }
///
/// assert_eq!(UserAccount::size(), 40);       // Raw size
/// assert_eq!(UserAccount::size_padded(), 48); // Including discriminator
/// ```
pub trait SizedData {
    /// Returns the raw size of the type in bytes
    fn size() -> usize;

    /// Returns the size including Anchor's 8-byte discriminator
    fn size_padded() -> usize {
        8 + Self::size()
    }
}

impl SizedData for u64 {
    fn size() -> usize {
        8
    }
}

impl SizedData for [u8; 32] {
    fn size() -> usize {
        32
    }
}

impl SizedData for Pubkey {
    fn size() -> usize {
        32
    }
}

impl SizedData for u8 {
    fn size() -> usize {
        1
    }
}