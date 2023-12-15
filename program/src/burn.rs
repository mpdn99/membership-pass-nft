use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	Subscription,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` mint: [Mint] 
/// 2. `[writable]` subscription: [Subscription] 
/// 3. `[writable]` account: [Account] The account to burn from.
/// 4. `[signer]` owner: [AccountInfo] The account's owner/delegate.
/// 5. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 6. `[]` token_program: [AccountInfo] SPL Token program
/// 7. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - plan: [Pubkey] 
pub fn burn(
	program_id: &Pubkey,
	for_burn: &[&AccountInfo],
	mint: &Account<spl_token::state::Mint>,
	subscription: &mut AccountPDA<Subscription>,
	account: &AccountPDA<spl_token::state::Account>,
	owner: &AccountInfo,
	wallet: &AccountInfo,
	plan: Pubkey,
) -> ProgramResult {
	csl_spl_token::src::cpi::burn(
		for_burn,
		1,
	)?;
	
    Ok(())
}