use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// Declare the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint implementation
pub fn process_instruction(
    _program_id: &Pubkey,       // Public key of the program
    _accounts: &[AccountInfo],  // Accounts involved in the transaction
    _instruction_data: &[u8],   // Instruction data
) -> ProgramResult {
    msg!("Hello, Solana!");    // Log message to the blockchain
    Ok(())
}
