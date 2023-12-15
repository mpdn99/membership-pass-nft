use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, clock::Clock};
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	Membership,
	Subscription,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable, signer]` mint: [Mint] 
/// 2. `[]` membership: [Membership] 
/// 3. `[writable]` subscription: [Subscription] 
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 5. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 6. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 7. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 8. `[]` token_program: [AccountInfo] SPL Token program
/// 9. `[signer]` owner: [AccountInfo] The mint's minting authority.
/// 10. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
/// 11. `[]` csl_spl_assoc_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
///
/// Data:
/// - plan: [Pubkey] 
pub fn mint(
	program_id: &Pubkey,
	for_initialize_mint_2: &[&AccountInfo],
	for_create: &[&AccountInfo],
	for_mint_to: &[&AccountInfo],
	for_set_authority: &[&AccountInfo],
	mint: &Account<spl_token::state::Mint>,
	membership: &AccountPDA<Membership>,
	subscription: &mut AccountPDA<Subscription>,
	funding: &AccountInfo,
	assoc_token_account: &AccountInfo,
	wallet: &AccountInfo,
	owner: &AccountInfo,
	plan: Pubkey,
) -> ProgramResult {
	let clock = Clock::get()?;
	let now = clock.unix_timestamp as u64;
	let expiry = now + 30 * 24 * 60 * 60;
	subscription.data.name = membership.data.name.clone();
	subscription.data.short_description = membership.data.short_description.clone();
	subscription.data.expiry = expiry;
	subscription.data.mint = *mint.info.key;
	subscription.data.assoc_account = Some(*assoc_token_account.key);
	
	csl_spl_token::src::cpi::initialize_mint_2(
		for_initialize_mint_2,
		0,
		*wallet.key,
		None,
	)?;

	csl_spl_assoc_token::src::cpi::create(
		for_create,
	)?;

	csl_spl_token::src::cpi::mint_to(
		for_mint_to,
		1,
	)?;

	csl_spl_token::src::cpi::set_authority(
		for_set_authority,
		0,
		Some(*owner.key),
	)?;

    Ok(())
}