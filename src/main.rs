
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

    fn find_index(accounts:&[Account],address:u32)->Option<usize>{
        for (index,acc) in accounts.iter().enumerate(){
            if acc.address==address{
                return Some(index)
            }
        }
        None
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

    fn process_instruction(instruction:Instruction,accounts:&mut[Account])->Result<(),String>{
        let from_ad_index = Account::find_index(&accounts, instruction.from_address).ok_or("No from id".to_string())?;
        let to_ad_index = Account::find_index(&accounts, instruction.to_address).ok_or("No to id".to_string())?;
        if from_ad_index==to_ad_index{
            return Err("From and to addresses cannot be the same".to_string());
        }
        if instruction.amount<=0{
            return Err("Amount cannot be negative".to_string());
        }
        if instruction.amount>accounts[from_ad_index].lamports{
            return Err("Insufficient Balance in Lamports".to_string());
        }
        if instruction.program==ProgramType::System{
            println!("Program Type: {:?}",instruction.program);
            println!("From: {} and To: {}",instruction.from_address,instruction.to_address);
            println!("Transfer Amount: {}",instruction.amount);
            accounts[from_ad_index].lamports-=instruction.amount;
            accounts[to_ad_index].lamports+=instruction.amount;
            return Ok(());
            
           
        }
        else {
            return Err("Only Program Type System eligible for transfer".to_string());
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
    let first_instruction = Instruction{program:ProgramType::System,from_address:2,to_address:1,amount:-1};
    let second_instruciton = Instruction{program:ProgramType::Other,from_address:2,to_address:1,amount:129};
    let first_test = Account::process_instruction(first_instruction, &mut accounts);
    match first_test {
        Ok(value)=>println!("{:?}",value),
        Err(msg)=>println!("{}",msg),
    }
    Account::print_state(&accounts);
   
    Ok(())
}