// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug)]
pub struct Account<'a, 'b, T> {
    pub data: T,
    pub info: &'a AccountInfo<'b>,
}

#[derive(Clone, Debug)]
pub struct AccountPDA<'a, 'b, T> {
    pub data: T,
    pub info: &'a AccountInfo<'b>,
    pub bump: u8,
}

impl<'a, 'b, T> Account<'a, 'b, T> {
    pub fn new(info: &'a AccountInfo<'b>, data: T) -> Self {
        Self { data, info }
    }
}

impl<'a, 'b, T> AccountPDA<'a, 'b, T> {
    pub fn new(info: &'a AccountInfo<'b>, data: T, bump: u8) -> Self {
        Self { data, info, bump }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct Membership {
	pub name: String,
	pub short_description: String,
	pub price: u64,
}

impl Membership {
	pub const LEN: usize = 287; 
	}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct Subscription {
	pub name: String,
	pub short_description: String,
	pub expiry: u64,
	pub mint: Pubkey,
	pub assoc_account: Option<Pubkey>,
}

impl Subscription {
	pub const LEN: usize = 352; 
	}
