use solana_program::{
    address_lookup_table::instruction::{create_lookup_table, extend_lookup_table},
    address_lookup_table::state::AddressLookupTable,
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    declare_id,
    msg,
};

declare_id!("2opr1VoyXxpNePA4gcLBGPMPgrzgpyixuqDrE7EzKFWv");
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("ALT Updater Program ID: {:?}", program_id);

    // expected accounts:
    //     0. ALT account
    //     1. authority
    //     2. payer
    //     3. system program
    //     4. ALT program
    if accounts.len() < 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let acc_iter = &mut accounts.iter();
    let acc_lookup_table = next_account_info(acc_iter)?;
    let acc_authority = next_account_info(acc_iter)?;
    let acc_payer = next_account_info(acc_iter)?;
    let acc_system_program = next_account_info(acc_iter)?;
    let acc_alt_program = next_account_info(acc_iter)?;

    // consider all onward accounts as an extends for ALT table
    let acc_extends = acc_iter.as_slice();

    // take Recent Slot from instruction data
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let recent_slot = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());

    // create new ALT on acc_lookup_table if it doesn't contain a correct ALT already
    if AddressLookupTable::deserialize(&mut acc_lookup_table.data.borrow_mut()).is_err() {
        let (ix_create, _) = create_lookup_table(
            acc_authority.key.clone(),
            acc_payer.key.clone(),
            recent_slot, // recent slot for the lookup table
        );
        invoke(
            &ix_create,
            &[
                acc_lookup_table.clone(),
                acc_authority.clone(),
                acc_payer.clone(),
                acc_system_program.clone(),
                acc_alt_program.clone(),
            ])?;
    }

    // extend ALT
    if acc_extends.len() > 0 {
        let pk_extends: Vec<_> = acc_extends.iter().map(|acc| *acc.key).collect();
        let ix_extend = extend_lookup_table(
            acc_lookup_table.key.clone(),
            acc_authority.key.clone(),
            Some(acc_payer.key.clone()),
            pk_extends
        );
        invoke(
            &ix_extend,
            &[
                acc_lookup_table.clone(),
                acc_authority.clone(),
                acc_payer.clone(),
                acc_system_program.clone(),
                acc_alt_program.clone(),
            ],
        )?;
    }

    Ok(())
}
