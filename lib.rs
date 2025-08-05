use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Todos {
    todos: Vec<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Operation {
    add_todo(String),
    remove_todo(String),
    update_todo(String, String),
}
// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    _instruction_data: &[u8], 
) -> ProgramResult {
let acc = next_account_info(&mut accounts.iter())?;

    let mut user_account = Todos::try_from_slice(&acc.data.borrow())?;

    let instruction_data = Operation::try_from_slice(&_instruction_data)?;

    match (instruction_data) {
        Operation::add_todo(todo) => {
            user_account.todos.push(todo);
        }
        Operation::remove_todo(todo) => {
            user_account.todos.retain(|todos| todos != &todo);
        }
        Operation::update_todo(old_todo, new_todo) => {
            if let Some(pos) = user_account.todos.iter().position(|todo| old_todo == *todo) {
                user_account.todos[pos] = new_todo;
            }
        }
    }
     user_account.serialize(&mut *acc.data.borrow_mut())?;

    Ok(())
}