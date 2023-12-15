use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, clock::Clock};
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	Subscription,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` mint: [Mint] 
/// 2. `[writable]` subscription: [Subscription] 
///
/// Data:
/// - plan: [Pubkey] 
pub fn extend_expiry(
	program_id: &Pubkey,
	mint: &Account<spl_token::state::Mint>,
	subscription: &mut AccountPDA<Subscription>,
	plan: Pubkey,
) -> ProgramResult {
	let clock = Clock::get()?;
	let now = clock.unix_timestamp as u64;
	let expiry = now + 30 * 24 * 60 * 60;
	subscription.data.expiry = expiry;

    Ok(())
}