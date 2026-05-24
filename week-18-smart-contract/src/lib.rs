use solana_program::pubkey::Pubkey;

enum ExpressionType{
    Sum,
    Sub
}

struct AccountInfo{
    pubkey: Pubkey,
    is_signer: bool
}
pub fn process_instruction(expression_type: ExpressionType, counter_acc_pub_key: AccountInfo){

    // check id the counter account signed the transaction
    if !counter_acc_pub_key.is_signer{
        return;
    }

    // read the data inside the counter account, deserialize it to a struct
    let current_data = blockchain.read(counter_acc_pub_key);

    // increase the value / decrease the value based on what user has requested
    match expression_type{
        ExpressionType::Sum { current_data.counter = current_data.counter + 1};
        ExpressionType::Sub { current_data.counter = current_data.counter - 1};

    }

    // write the change back
    blockchain.write(counter_acc_pub_key, current_data);
}
