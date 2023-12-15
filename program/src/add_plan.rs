use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	Membership,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` membership: [Membership] 
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - plan: [Pubkey] 
/// - name: [String] 
/// - short_description: [String] 
/// - price: [u64] 
pub fn add_plan(
	program_id: &Pubkey,
	membership: &mut AccountPDA<Membership>,
	plan: Pubkey,
	name: String,
	short_description: String,
	price: u64,
) -> ProgramResult {
	membership.data.name = name;
	membership.data.short_description = short_description;
	membership.data.price = price;

    Ok(())
}