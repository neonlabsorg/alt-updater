use solana_program::{
    address_lookup_table::instruction::{create_lookup_table, extend_lookup_table},
    address_lookup_table::state::AddressLookupTable,
    account_info::{AccountInfo, next_account_info},
    clock::Clock,
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    sysvar::Sysvar,
    declare_id,
    msg,
};

// Declare the program's ID
declare_id!("2opr1VoyXxpNePA4gcLBGPMPgrzgpyixuqDrE7EzKFWv");
// Declare the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint implementation
pub fn process_instruction(
    program_id: &Pubkey,       // Public key of the program
    accounts: &[AccountInfo],  // Accounts involved in the transaction
    _instruction_data: &[u8],   // Instruction data
) -> ProgramResult {
    msg!("ALT Updater Program ID: {:?}", program_id);

    let acc_iter = &mut accounts.iter();
    let acc_lookup_table = next_account_info(acc_iter)?;
    let acc_authority = next_account_info(acc_iter)?;
    let acc_payer = next_account_info(acc_iter)?;
    let acc_system_program = next_account_info(acc_iter)?;
    let acc_extends = acc_iter.as_slice();

    if AddressLookupTable::deserialize(&mut acc_lookup_table.data.borrow_mut()).is_err() {
        // Create ALT
        let (ix_create, _) = create_lookup_table(
            acc_authority.key.clone(),
            acc_payer.key.clone(),
            Clock::get()?.slot);

        invoke(
            &ix_create,
            &[
                acc_lookup_table.clone(),
                acc_authority.clone(),
                acc_payer.clone(),
                acc_system_program.clone(),
            ])?;

    }

    // Extend ALT
    let pk_extends: Vec<_> = acc_extends.iter().map(|acc| *acc.key).collect();
    let ix_extend = extend_lookup_table(
        acc_lookup_table.key.clone(),
        acc_authority.key.clone(),
        Some(acc_payer.key.clone()),
        pk_extends);

    invoke(
        &ix_extend,
        &[
            acc_lookup_table.clone(),
            acc_authority.clone(),
            acc_payer.clone(),
            acc_system_program.clone(),
        ],
    )?;

    Ok(())
}
