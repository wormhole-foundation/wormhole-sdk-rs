use solana_program::{account_info::AccountInfo, program_error::ProgramError};

/// Close an account by transferring all its lamports to another account.
pub fn close_account(
    info: &AccountInfo,
    sol_destination: &AccountInfo,
) -> Result<(), ProgramError> {
    // Transfer tokens from the account to the sol_destination.
    let dest_starting_lamports = sol_destination.lamports();
    **sol_destination.lamports.borrow_mut() =
        dest_starting_lamports.checked_add(info.lamports()).unwrap();
    **info.lamports.borrow_mut() = 0;

    info.assign(&solana_program::system_program::id());
    info.realloc(0, false).map_err(Into::into)
}