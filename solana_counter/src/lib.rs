use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};

const MAX_CLAIMS: u64 = 5; // Each user can claim up to 5 times

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;

    // User's claim count is stored in their account data (first 8 bytes)
    let mut data = user_account.try_borrow_mut_data()?;
    if data.len() < 8 {
        return Err(ProgramError::InvalidAccountData);
    }
    let mut claim_count = u64::from_le_bytes(data[0..8].try_into().unwrap());

    // Enforce the claim limit
    if claim_count >= MAX_CLAIMS {
        msg!("Claim limit reached: {} >= {}", claim_count, MAX_CLAIMS);
        return Err(ProgramError::Custom(0)); // Custom error code for limit reached
    }

    // Increment claim count and save back to account data
    claim_count += 1;
    data[0..8].copy_from_slice(&claim_count.to_le_bytes());

    // Here, you would add your airdrop/reward logic (e.g., transfer tokens/SOL)
    msg!("User successfully claimed reward #{}", claim_count);

    Ok(())
}
