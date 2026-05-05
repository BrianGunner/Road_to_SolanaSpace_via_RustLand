
#[derive(PartialEq)]
#[derive(Debug)]
enum ProgramType{
    System,Token,Other
}
#[derive(PartialEq)]

struct Account{
    address:u32,
    lamports:i32,
    owner:ProgramType
}

struct Instruction{
    from_ad:u32,
    to_ad:u32,
    amount:i32,
    program:ProgramType,
}

impl Account{
    fn create_new(accounts:&mut Vec<Account>,address:u32,program:ProgramType)->Result<(),String>{
        for acc in accounts.iter(){
            if acc.address==address{
                return Err("Account id already exits".to_string());
            }
        }
        accounts.push(Account { address: address, lamports:100, owner:program });
        Ok(())
    }

    fn print_state(accounts:&[Account]){
        for acc in accounts.iter(){
            println!("Address:{},Lamports:{},Owner:{:?}",acc.address,acc.lamports,acc.owner)
        }
    }

    fn find_index(accounts:&[Account],address:u32)->Option<usize>{
        for (index,acc) in accounts.iter().enumerate(){
            if acc.address==address{
                return Some(index)
            }
        }
        None
    }
    
    fn process_instruction(accounts:&[Account],instruction:Instruction)->Result<(),String>{
        let from_index = Account::find_index(&accounts, instruction.from_ad).ok_or("Could not find from index".to_string())?;
        let to_index = Account::find_index(accounts, instruction.to_ad).ok_or("Could not find to index")?;
        if instruction.program==ProgramType::System{
        if instruction.amount<=0{
            return Err("Tx amount cannot be less than or equal to 0".to_string());
        }
        if instruction.amount<accounts[from_index].lamports{
            return Err("Insufficient Balance".to_string());
        }
        if accounts[from_index]==accounts[to_index]{
            return Err("From and to id cannot be the same".to_string());
        }
        println!("Instruction is ready to be processed");
        Ok(())
    }
    else{
        return Err("Instruction must be of type System to be processed".to_string());
    }
        
    }
}

fn main(){

    let mut accounts: Vec<Account> = Vec::new();
    let account1 = Account::create_new(&mut accounts, 1, ProgramType::System);
    let account2 = Account::create_new(&mut accounts, 2, ProgramType::Other);
    match account2{
        Ok(value)=>println!("Account created"),
        Err(msg)=>println!("{}",msg),
    }
    let instruction_1=Instruction{from_ad:1,to_ad:2,amount:100,program:ProgramType::Token};
    let process_1 = Account::process_instruction(&accounts, instruction_1);
    match process_1 {
        Ok(value)=>println!("{:?}",value),
        Err(msg)=>println!("{}",msg),
    }


}