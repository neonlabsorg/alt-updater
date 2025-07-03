use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    pubkey::{PUBKEY_BYTES, Pubkey, find_program_address},
    cpi::invoke,
    entrypoint,
    program_error::ProgramError,
    ProgramResult,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // expected accounts:
    //     0. ALT account
    //     1. payer
    //     2. system program
    //     3. ALT program
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let acc_iter = &mut accounts.iter();
    let acc_lookup_table = acc_iter.next().unwrap();
    let acc_payer = acc_iter.next().unwrap();
    let acc_system_program = acc_iter.next().unwrap();
    let acc_alt_program = acc_iter.next().unwrap();
    let acc_extends = acc_iter.as_slice();

    // instruction accounts, the same for both CreateLookupTable and ExtendLookupTable
    let account_metas: [AccountMeta; 4] = [
        AccountMeta::writable(acc_lookup_table.key()),
        AccountMeta::readonly_signer(acc_payer.key()),
        AccountMeta::writable_signer(acc_payer.key()),
        AccountMeta::readonly(acc_system_program.key()),
    ];

    // create new ALT on acc_lookup_table account if it is empty
    if acc_lookup_table.data_is_empty() {
        // take Recent Slot from instruction data
        if instruction_data.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let recent_slot = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());

        // retrieve Bump Seed
        let (_lookup_table, bump_seed)  = find_program_address(
            &[acc_payer.key().as_ref(), &recent_slot.to_le_bytes()],
            acc_alt_program.key()
        );

        // CreateLookupTable instruction data
        //     [0..4 ]: Instruction discriminator, u32 (0 for CreateLookupTable)
        //     [4..12]: Recent Slot, u64
        //     [12   ]: bump seed, u8
        let mut instruction_data = [0u8; 13];
        // instruction_data[0..4] are zeroes
        instruction_data[4..12].copy_from_slice(&recent_slot.to_le_bytes()[..]);
        instruction_data[12] = bump_seed;

        let instruction = Instruction {
            program_id: &acc_alt_program.key(),
            accounts: &account_metas,
            data: &instruction_data,
        };
        invoke(
            &instruction,
            &[
                acc_lookup_table,
                acc_payer,
                acc_payer,
                acc_system_program
            ]
        )?
    }

    // extend ALT
    if acc_extends.len() > 0 {
        const MAX_EXTEND_ACCOUNTS: usize = 27;

        if acc_extends.len() > MAX_EXTEND_ACCOUNTS {
            return Err(ProgramError::MaxAccountsDataAllocationsExceeded);
        }

        // ExtendLookupTable instruction data
        //     [0..4 ]: Instruction discriminator, u32 (2 for ExtendLookupTable)
        //     [4..12]: pubkeys count, u64
        //     [12 + 32*i..12 + 32*(i+1)]: n-th Pubkey for extend
        let mut instruction_data = [0u8; 4 + 8 + MAX_EXTEND_ACCOUNTS * PUBKEY_BYTES];
        instruction_data[0] = 2;
        instruction_data[4..12].copy_from_slice(&acc_extends.len().to_le_bytes()[..]);

        for (idx, account) in acc_extends.iter().enumerate() {
            let offset = 4 + 8 + idx * PUBKEY_BYTES;
            instruction_data[offset..offset + PUBKEY_BYTES]
                .copy_from_slice(account.key().as_ref());
        }
        let instruction = Instruction {
            program_id: &acc_alt_program.key(),
            accounts: &account_metas,
            data: &instruction_data[..4 + 8 + acc_extends.len() * PUBKEY_BYTES],
        };
        invoke(
            &instruction,
            &[
                acc_lookup_table,
                acc_payer,
                acc_payer,
                acc_system_program
            ]
        )?
    }

    Ok(())
}
