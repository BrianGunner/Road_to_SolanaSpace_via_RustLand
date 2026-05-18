#[derive(Debug)]
#[derive(PartialEq)]enum ProgramType{
    System,
    Token,
    Other,
}
#[derive(Debug)]
#[derive(PartialEq)]
struct TxInstruction{
    from_address:u32,
    to_address:u32,
    amount:i32,
    program:ProgramType,
}


impl TxInstruction{
    fn create_instruction(from_address:u32,to_address:u32,amount:i32,program:ProgramType)->TxInstruction{
        TxInstruction { from_address, to_address, amount, program }
    }

    fn validate_instruction(accounts:&[Account],instruction:&TxInstruction)->Result<Plan,String>{
        if instruction.program!=ProgramType::System{
            return Err("Not System Program".to_string())
        }
        let from_index=Account::find_index(&accounts, instruction.from_address).ok_or("From id not found".to_string())?;
        let to_index = Account::find_index(&accounts, instruction.to_address).ok_or("To id not found".to_string())?;

        if from_index==to_index{
            return Err("From id cannot be same as to id".to_string());
        }
        if accounts[from_index].lamports<instruction.amount{
            return Err("Insufficient Balance".to_string());
        }
        if instruction.amount<=0{
            return Err("Transfer amount should be more than zero".to_string());
        }

        if accounts[from_index].owner!=ProgramType::System{
            return Err("From Account owner not System".to_string());
        }
        if accounts[to_index].owner!=ProgramType::System{
            return Err("To id owner not System".to_string());
        }
        Ok(Plan { from_index, to_index, amount: instruction.amount })
    }
}

struct Account{
    address:u32,
    lamports:i32,
    owner:ProgramType,
}
#[derive(PartialEq)]
#[derive(Debug)]
struct Plan{
    from_index:usize,
    to_index:usize,
    amount:i32,
}

impl Plan{
    fn execute_plan(accounts:&mut [Account],plan_output:Plan){
        accounts[plan_output.from_index].lamports-=plan_output.amount;
        accounts[plan_output.to_index].lamports+=plan_output.amount;
    }
}

impl Account{
    fn create_account(accounts:&mut Vec<Account>,address:u32,owner:ProgramType)->Result<(),String>{
        for acc in accounts.iter(){
            if acc.address==address{
                return Err("Account address already exists".to_string());
            }
        }
        accounts.push(Account { address, lamports: 100, owner });
        Ok(())
    }

    fn print_state(accounts:&[Account]){
        for acc in accounts.iter(){
            println!("Address: {},Balance: {}, Owner: {:?}",acc.address,acc.lamports,acc.owner)
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

}

struct RunTime;

impl RunTime{
    fn process_instruction(accounts:&mut [Account],instruction:&TxInstruction)->Result<(),String>{
        let plan_exec = TxInstruction::validate_instruction(&accounts, instruction)?;
        Plan::execute_plan(accounts, plan_exec);
        Ok(())
        
    }
}

fn main()->Result<(),String>{
    let mut accounts: Vec<Account> = Vec::new();
    Account::create_account(&mut accounts, 1, ProgramType::System);
    Account::create_account(&mut accounts, 2, ProgramType::Other);
    Account::create_account(&mut accounts, 3, ProgramType::Token);
    Account::create_account(&mut accounts, 4, ProgramType::System);
    Account::create_account(&mut accounts, 5, ProgramType::Other);
    Account::create_account(&mut accounts, 6, ProgramType::Token);
    Account::print_state(&accounts);
    let first_inst = TxInstruction::create_instruction(1, 4, 10, ProgramType::System);
    let secon_inst = TxInstruction::create_instruction(13, 6, 199, ProgramType::System);
    let first_test = RunTime::process_instruction(&mut accounts, &secon_inst);
    match first_test{
        Ok(value)=>println!("Done"),
        Err(msg)=>println!("{}",msg),
    }
    Ok(())
    
}
