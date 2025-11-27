use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::create_account,
    system_program,
};

entrypoint!(program_instruction);


pub fn program_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();

    let user_account = next_account_info(&mut iter)?;
    let pda_account = next_account_info(&mut iter)?;
    let system_program_account = next_account_info(&mut iter)?;

    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !pda_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    if system_program_account.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let seeds = &[b"user", user_account.key.as_ref()];
    let (pda_pubkey, bump) = Pubkey::find_program_address(seeds, program_id);

    if pda_pubkey != *pda_account.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let signer_seeds: &[&[u8]] = &[
        b"user",
        user_account.key.as_ref(),
        &[bump],
    ];
    let signer_seed_slice: &[&[&[u8]]] = &[signer_seeds];

    let ix = create_account(
        user_account.key,
        pda_account.key,
        1_000_000_000, 
        8,
        program_id,
    );


    invoke_signed(&ix, &accounts[0..3], signer_seed_slice)?;

    Ok(())
}
