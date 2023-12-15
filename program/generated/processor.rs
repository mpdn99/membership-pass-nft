// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use std::str::FromStr;
use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info, next_account_infos};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::{msg, system_program};
use solana_program::sysvar::Sysvar;
use solana_program::program_pack::Pack;
use crate::generated::errors::NftMembershipError;
use crate::generated::instructions::NftMembershipInstruction;

use crate::generated::state::{
	Account,
	AccountPDA,
	Membership,
	Subscription,
};
use crate::src::*;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let instruction = NftMembershipInstruction::unpack(data)?;

        match instruction {
			NftMembershipInstruction::AddPlan(args) => {
				msg!("Instruction: AddPlan");
				Self::process_add_plan(
					program_id,
					accounts, 
					args.plan,
					args.name,
					args.short_description,
					args.price,
				)
			}
			NftMembershipInstruction::Mint(args) => {
				msg!("Instruction: Mint");
				Self::process_mint(
					program_id,
					accounts, 
					args.plan,
				)
			}
			NftMembershipInstruction::Transfer(args) => {
				msg!("Instruction: Transfer");
				Self::process_transfer(
					program_id,
					accounts, 
					args.plan,
				)
			}
			NftMembershipInstruction::Burn(args) => {
				msg!("Instruction: Burn");
				Self::process_burn(
					program_id,
					accounts, 
					args.plan,
				)
			}
			NftMembershipInstruction::ExtendExpiry(args) => {
				msg!("Instruction: ExtendExpiry");
				Self::process_extend_expiry(
					program_id,
					accounts, 
					args.plan,
				)
			}
        }
    }

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
	pub fn process_add_plan(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		plan: Pubkey,
		name: String,
		short_description: String,
		price: u64,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let membership_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (membership_pubkey, membership_bump) = Pubkey::find_program_address(
			&[b"membership", plan.as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *membership_info.key != membership_pubkey {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = Membership::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&membership_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), membership_info.clone()],
			&[&[b"membership", plan.as_ref(), &[membership_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if membership_info.data_len() != Membership::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let membership = &mut AccountPDA::new(
			&membership_info,
			try_from_slice_unchecked::<Membership>(&membership_info.data.borrow()).unwrap(),
			membership_bump,
		);

		// Calling STUB
		add_plan::add_plan(
			program_id,
			membership,
			plan,
			name,
			short_description,
			price,
		)?;

		// Accounts Serialization
		membership.data.serialize(&mut &mut membership_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_mint(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		plan: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let membership_info = next_account_info(account_info_iter)?;
		let subscription_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;
		let funding_info = next_account_info(account_info_iter)?;
		let assoc_token_account_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let owner_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;
		let csl_spl_assoc_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (membership_pubkey, membership_bump) = Pubkey::find_program_address(
			&[b"membership", plan.as_ref()],
			program_id,
		);
		let (subscription_pubkey, subscription_bump) = Pubkey::find_program_address(
			&[b"subscription", plan.as_ref(), mint_info.key.as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if mint_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *membership_info.key != membership_pubkey {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if *subscription_info.key != subscription_pubkey {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if funding_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if owner_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if *csl_spl_assoc_token_v_0_0_0_info.key != Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = spl_token::state::Mint::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke(
			&create_account(
				&fee_payer_info.key,
				&mint_info.key,
				rent_minimum_balance,
				space as u64,
				&Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
			),
			&[fee_payer_info.clone(), mint_info.clone()],
		)?;

		let space = Subscription::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&subscription_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), subscription_info.clone()],
			&[&[b"subscription", plan.as_ref(), mint_info.key.as_ref(), &[subscription_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if *funding_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}

		if membership_info.data_len() != Membership::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}

		if subscription_info.data_len() != Subscription::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let membership = AccountPDA::new(
			&membership_info,
			try_from_slice_unchecked::<Membership>(&membership_info.data.borrow()).unwrap(),
			membership_bump,
		);
		let subscription = &mut AccountPDA::new(
			&subscription_info,
			try_from_slice_unchecked::<Subscription>(&subscription_info.data.borrow()).unwrap(),
			subscription_bump,
		);

		// Calling STUB
		mint::mint(
			program_id,
			&vec![
				mint_info,
			],
			&vec![
				funding_info,
				assoc_token_account_info,
				wallet_info,
				mint_info,
				system_program_info,
				token_program_info,
			],
			&vec![
				mint_info,
				assoc_token_account_info,
				owner_info,
				wallet_info,
				token_program_info,
			],
			&vec![
				mint_info,
				owner_info,
			],
			&mint,
			&membership,
			subscription,
			funding_info,
			assoc_token_account_info,
			wallet_info,
			owner_info,
			plan,
		)?;

		// Accounts Serialization
		subscription.data.serialize(&mut &mut subscription_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` mint: [Mint] 
/// 2. `[writable]` subscription: [Subscription] 
/// 3. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 4. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 5. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 6. `[]` system_program: [AccountInfo] System program
/// 7. `[]` token_program: [AccountInfo] SPL Token program
/// 8. `[writable]` source: [AccountInfo] The source account.
/// 9. `[writable]` destination: [AccountInfo] The destination account.
/// 10. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 11. `[]` csl_spl_assoc_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
/// 12. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - plan: [Pubkey] 
	pub fn process_transfer(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		plan: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let subscription_info = next_account_info(account_info_iter)?;
		let funding_info = next_account_info(account_info_iter)?;
		let assoc_token_account_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let source_info = next_account_info(account_info_iter)?;
		let destination_info = next_account_info(account_info_iter)?;
		let authority_info = next_account_info(account_info_iter)?;
		let csl_spl_assoc_token_v_0_0_0_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (subscription_pubkey, subscription_bump) = Pubkey::find_program_address(
			&[b"subscription", plan.as_ref(), mint_info.key.as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *subscription_info.key != subscription_pubkey {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if funding_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if authority_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *csl_spl_assoc_token_v_0_0_0_info.key != Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if *funding_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}

		if subscription_info.data_len() != Subscription::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let subscription = &mut AccountPDA::new(
			&subscription_info,
			try_from_slice_unchecked::<Subscription>(&subscription_info.data.borrow()).unwrap(),
			subscription_bump,
		);

		// Calling STUB
		transfer::transfer(
			program_id,
			&vec![
				funding_info,
				assoc_token_account_info,
				wallet_info,
				mint_info,
				system_program_info,
				token_program_info,
			],
			&vec![
				source_info,
				mint_info,
				destination_info,
				authority_info,
			],
			&mint,
			subscription,
			funding_info,
			assoc_token_account_info,
			wallet_info,
			source_info,
			destination_info,
			authority_info,
			plan,
		)?;

		// Accounts Serialization
		subscription.data.serialize(&mut &mut subscription_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_burn(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		plan: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let subscription_info = next_account_info(account_info_iter)?;
		let account_info = next_account_info(account_info_iter)?;
		let owner_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (subscription_pubkey, subscription_bump) = Pubkey::find_program_address(
			&[b"subscription", plan.as_ref(), mint_info.key.as_ref()],
			program_id,
		);
		let (account_pubkey, account_bump) = Pubkey::find_program_address(
			&[wallet_info.key.as_ref(), token_program_info.key.as_ref(), mint_info.key.as_ref()],
			&Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *subscription_info.key != subscription_pubkey {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if *account_info.key != account_pubkey {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if owner_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}

		if subscription_info.data_len() != Subscription::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}

		if account_info.data_len() != spl_token::state::Account::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let subscription = &mut AccountPDA::new(
			&subscription_info,
			try_from_slice_unchecked::<Subscription>(&subscription_info.data.borrow()).unwrap(),
			subscription_bump,
		);
		let account = AccountPDA::new(
			&account_info,
			spl_token::state::Account::unpack_from_slice(&account_info.data.borrow()).unwrap(),
			account_bump,
		);

		// Calling STUB
		burn::burn(
			program_id,
			&vec![
				account_info,
				mint_info,
				owner_info,
				wallet_info,
				token_program_info,
			],
			&mint,
			subscription,
			&account,
			owner_info,
			wallet_info,
			plan,
		)?;

		// Accounts Serialization
		subscription.data.serialize(&mut &mut subscription_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` mint: [Mint] 
/// 2. `[writable]` subscription: [Subscription] 
///
/// Data:
/// - plan: [Pubkey] 
	pub fn process_extend_expiry(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		plan: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let subscription_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (subscription_pubkey, subscription_bump) = Pubkey::find_program_address(
			&[b"subscription", plan.as_ref(), mint_info.key.as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftMembershipError::InvalidSignerPermission.into());
		}

		if *subscription_info.key != subscription_pubkey {
			return Err(NftMembershipError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftMembershipError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}

		if subscription_info.data_len() != Subscription::LEN {
			return Err(NftMembershipError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let subscription = &mut AccountPDA::new(
			&subscription_info,
			try_from_slice_unchecked::<Subscription>(&subscription_info.data.borrow()).unwrap(),
			subscription_bump,
		);

		// Calling STUB
		extend_expiry::extend_expiry(
			program_id,
			&mint,
			subscription,
			plan,
		)?;

		// Accounts Serialization
		subscription.data.serialize(&mut &mut subscription_info.data.borrow_mut()[..])?;
		
		Ok(())
	}
}