use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey, system_instruction::create_account};

fn program_instruction(
program_id: &Pubkey,
accounts: &[AccountInfo],
instructon_data:&[u8]
)->ProgramResult{

let mut iter = accounts.iter();

let pda_account = next_account_info(&mut iter)?;
let user_account = next_account_info(&mut iter)?;
let system_program = next_account_info(&mut iter)?;

let seeds = &[user_account.key.as_ref(), b"user"];

let (pda_pubkey, bump) = Pubkey::find_program_address(seeds, program_id);

let ix = create_account(
    user_account.key,
     pda_account.key,
      1000000000,
       8,
        program_id
    );


invoke_signed(&ix, accounts, &[seeds, &[&[bump]]])


}