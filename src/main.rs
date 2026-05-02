
#[derive(Debug)]
#[derive(PartialEq)]
enum ProgramType {
    System,
    Token,
    Other,
}

struct Instruction{
    program:ProgramType,
    from_address:u32,
    to_address:u32,
    amount:i32,
}
struct Account{
    address:u32,
    lamports:i32,
    owner:ProgramType,
}



impl Account {
    fn print_state(accounts:&[Account]){
        println!("🤑+++++++++++🤑");
        for acc in accounts.iter(){
            println!("Address: {}, Lamports: {}, Owner: {:?}",acc.address,acc.lamports,acc.owner)
        }
        println!("🤑+++++++++++🤑");
    }

    fn create_account(accounts:&mut Vec<Account>,address:u32)->Result<(),String>{
        for acc in accounts.iter(){
            if acc.address == address{
                return Err("Address already exists".to_string());
            }
        }
        accounts.push(Account { address:address, lamports: 100, owner: ProgramType::System});
        Ok(())
    
    }

    fn process_instruction(instruction:Instruction){
        if instruction.program==ProgramType::System{
            println!("Program Type: {:?}",instruction.program);
            println!("From: {} and To: {}",instruction.from_address,instruction.to_address);
            println!("Transfer Amount: {}",instruction.amount);
        }
        else{
            println!("Only Program Type System eligible for transfer")
        }
    }


}

fn main()->Result<(),String>{
    let mut accounts = Vec::new();
    Account::create_account(&mut accounts, 1)?;
    Account::create_account(&mut accounts, 2)?;  
    Account::create_account(&mut accounts, 3)?;
    Account::create_account(&mut accounts, 4)?;
    Account::print_state(&accounts);
    let first_instruction = Instruction{program:ProgramType::System,from_address:1,to_address:2,amount:100};
    let second_instruciton = Instruction{program:ProgramType::Other,from_address:2,to_address:3,amount:129};
    Account::process_instruction(first_instruction);
    Account::process_instruction(second_instruciton);
    Ok(())
}