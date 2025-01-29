use anchor_lang::prelude::Pubkey;

pub use sized_data_derive::*;

pub trait SizedData {
    fn size() -> usize;
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
