use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::{self, ProgramResult};
use solana_program::account_info::{AccountInfo};

// enum ExpressionType{
//     Sum,
//     Sub
// }

// struct AccountInfo{
//     pubkey: Pubkey,
//     is_signer: bool
// }

#[derive(BorshSerialize, BorshDeserialize)]
struct Counter{
    count: u32
}
#[derive(BorshSerialize, BorshDeserialize)] 
enum InstructionData{
    Increase,
    Decrease
}
entrypoint!(process_instruction);
// process_instruction takes 3 input as argument
pub fn process_instruction(
    _pubkey: &Pubkey, //public key of where this program is deployed
    account: &[AccountInfo], // array of all the account you're going to read from or write to in this transaction: read from the counter account [Counter account]
    instruction_data: &[u8] // actual thing the user wants to do, increase or decrease
    /// bytes: [00-0010......................] => {
    /// }
    /// 
    // Why? array of AccountInfo: terminal, @solana/web3.js, rust
    // They sends three things: 1. programId 2. intruction_data-(decimal), 3. Accounts[]
    // parallelized transaction in solana


    // instuction to struct
) -> ProgramResult{
    // 1. Check if the counter account has signed the transaction
    // How do I get 0th index
    // create an iterator
    let mut iter = account.iter(); // iter give nice error
    let counter_account = next_account_info(&mut iter)?; // notenoughaccountkey error: ? 

    if !counter_account.is_signer{
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    let mut counter = Counter::try_from_slice(&counter_account.data.borrow())?; //Rc/RefCell 
    // counter returns Result : ?

    let instruction_data = InstructionData::try_from_slice(instruction_data)?;

    match instruction_data{
        InstructionData::Decrease => {
            counter.count = counter.count - 1;
        },
        InstructionData::Increase => {
            counter.count = counter.count + 1;
        } 
    }

    counter.serialize(&mut *counter_account.data.borrow_mut());

    Ok();

}
// pub fn process_instruction(expression_type: ExpressionType, counter_acc_pub_key: AccountInfo){

//     // check id the counter account signed the transaction
//     if !counter_acc_pub_key.is_signer{
//         return;
//     }

//     // read the data inside the counter account, deserialize it to a struct
//     let current_data = blockchain.read(counter_acc_pub_key);

//     // increase the value / decrease the value based on what user has requested
//     match expression_type{
//         ExpressionType::Sum { current_data.counter = current_data.counter + 1};
//         ExpressionType::Sub { current_data.counter = current_data.counter - 1};

//     }

//     // write the change back
//     blockchain.write(counter_acc_pub_key, current_data);
// }
