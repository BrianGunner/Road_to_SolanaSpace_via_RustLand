
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
#[derive(Debug)]
struct Plan{
    from_ind:usize,
    to_ind:usize,
    amt:i32,
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
        println!("🤑**************🤑");
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
    
    fn validate_system_instruction(accounts:&[Account],instruction:&Instruction)->Result<Plan,String>{
        
        if instruction.program!=ProgramType::System{
            return Err("Only System Program allowed".to_string())
            
        }
            
        let from_index=Account::find_index(&accounts,instruction.from_ad).ok_or("No from id".to_string())?;
        let to_index = Account::find_index(&accounts, instruction.to_ad).ok_or("No to id".to_string())?;
        if from_index==to_index{
            return Err("From and to ids cannot be the same".to_string());
        }
        if instruction.amount<=0{
            return Err("Tx amount cannot be less than or 0".to_string());
        }
        if accounts[from_index].lamports<instruction.amount{
            return Err("Insufficient Balance".to_string());
        }
        if accounts[from_index].owner!=ProgramType::System{
            return Err("From account owner not System".to_string());
        }
        if accounts[to_index].owner!=ProgramType::System{
            return Err("To account owner not System".to_string());
        }
        return Ok(Plan{from_ind:from_index,to_ind:to_index,amt:instruction.amount});

        

    }

    fn execute_plan(accounts:&mut[Account],plan:Plan){
        accounts[plan.from_ind].lamports-=plan.amt;
        accounts[plan.to_ind].lamports+=plan.amt;
    }
    
}

fn main()->Result<(),String>{

    let mut accounts: Vec<Account> = Vec::new();
    let account1 = Account::create_new(&mut accounts, 1, ProgramType::System);
    let account2 = Account::create_new(&mut accounts, 2, ProgramType::System);
    let account3 = Account::create_new(&mut accounts, 3, ProgramType::Token);
    let account4 = Account::create_new(&mut accounts, 4, ProgramType::Other);
    let instruction_1= Instruction{from_ad:2,to_ad:1,amount:100,program:ProgramType::System};
    let process_instruction_1=Account::validate_system_instruction(&accounts, &instruction_1);
    match process_instruction_1 {
        Ok(value)=>println!("{:?}",value),
        Err(msg)=>println!("{}",msg),
    }
    Account::print_state(&accounts);
    let plan_1 = Account::validate_system_instruction(&mut accounts, &instruction_1)?;
    Account::execute_plan(&mut accounts, plan_1);
    Account::print_state(&accounts);
    Ok(())
    

}