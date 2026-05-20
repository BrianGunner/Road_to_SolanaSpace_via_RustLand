

#[derive(Debug)]
#[derive(PartialEq)]
enum ProgramType{
    System,
    Token,
    Others,
}

struct Account{
    address:u32,
    lamports:u64,
    token_balance:u64,
    owner:ProgramType,
}
impl Account{
    fn create_new(accounts:&mut Vec<Account>,address:u32,lamports:u64,token_balance:u64,owner:ProgramType)->Result<(),String>{
        for acc in accounts.iter(){
            if acc.address==address{
                return Err("Address already exists".to_string());
            }
        }
        accounts.push(Account { address, lamports, token_balance, owner });
        Ok(())
    }
    fn print_account_state(accounts:&[Account]){
        println!("🤑+++++++++++++++++++++++🤑");
        for acc in accounts.iter(){
            println!("Address: {}, Lamports: {}, Token Balance: {}, Owner: {:?}",acc.address,acc.lamports,acc.token_balance,acc.owner)
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
struct TxInstruction{
    from_address:u32,
    to_address:u32,
    amount:u64,
    program:ProgramType,
}
impl TxInstruction{
    fn create_tx_inst(transactions:&mut Vec<TxInstruction>,from_address:u32,to_address:u32,amount:u64,program:ProgramType){
        transactions.push(TxInstruction { from_address, to_address, amount, program })
    }
    fn print_tx_state(transactions:&[TxInstruction]){
        println!("📒______________________📒");
        for txs in transactions.iter(){
            println!("From: {},To: {},Amount: {},Program: {:?}",txs.from_address,txs.to_address,txs.amount,txs.program)
        }
    }
    fn token_validator(accounts:&[Account],instruction:&TxInstruction)->Result<Plan,String>{
        if instruction.program!=ProgramType::Token{
            return Err("Program not Token".to_string())
        }
        let from_index = Account::find_index(&accounts, instruction.from_address).ok_or("from not found".to_string())?;
        let to_index = Account::find_index(&accounts, instruction.to_address).ok_or("to not found".to_string())?;
        if from_index==to_index{
            return Err("From id cannot be same as to id".to_string())
        }
        if accounts[from_index].token_balance<instruction.amount{
            return Err("Insufficient Balance".to_string());
        }
        if accounts[from_index].owner!=ProgramType::Token{
            return Err("From Account owner not System".to_string());
        }
        if accounts[to_index].owner!=ProgramType::Token{
            return Err("To id owner not System".to_string());
        }
        Ok(Plan { from_index, to_index, amount: instruction.amount,program:ProgramType::Token})
    }

    fn system_validator(accounts:&[Account],instruction:&TxInstruction)->Result<Plan,String>{
        if instruction.program!=ProgramType::System{
            return Err("Program not System".to_string())
        }
        let from_index = Account::find_index(&accounts, instruction.from_address).ok_or("from not found".to_string())?;
        let to_index = Account::find_index(&accounts, instruction.to_address).ok_or("to not found".to_string())?;
        if from_index==to_index{
            return Err("From id cannot be same as to id".to_string())
        }
        if accounts[from_index].lamports<instruction.amount{
            return Err("Insufficient Balance".to_string());
        }
        if accounts[from_index].owner!=ProgramType::System{
            return Err("From Account owner not System".to_string());
        }
        if accounts[to_index].owner!=ProgramType::System{
            return Err("To id owner not System".to_string());
        }
        Ok(Plan { from_index, to_index, amount: instruction.amount,program:ProgramType::System})
    }

    }
#[derive(Debug)]
struct Plan{
    from_index:usize,
    to_index:usize,
    amount:u64,
    program:ProgramType,
}

impl Plan{
    fn execute_system_plan(accounts:&mut[Account],plan_e:&Plan){
        accounts[plan_e.from_index].lamports-=plan_e.amount;
        accounts[plan_e.to_index].lamports+=plan_e.amount;
    }
    fn execute_token_plan(accounts:&mut[Account],plan_e:&Plan){
        accounts[plan_e.from_index].token_balance-=plan_e.amount;
        accounts[plan_e.to_index].token_balance+=plan_e.amount;
    }
}

struct RunTime;
impl RunTime{
    fn Runtime(accounts:&mut[Account],transactions:&[TxInstruction])->Result<(),String>{
        let mut plans = Vec::new();
        let ins_sys = &transactions[0];
        let ins_token = &transactions[1];
        match TxInstruction::system_validator(accounts, &ins_sys){
            Ok(value)=>plans.push(value),
            Err(msg)=>return Err(msg),
        }
        match TxInstruction::token_validator(accounts, &ins_token){
            Ok(value)=>plans.push(value),
            Err(msg)=>return Err(msg),
        }

        for plan in plans.iter(){
            if plan.program==ProgramType::System{
                Plan::execute_system_plan(accounts, plan);
            }
            if plan.program==ProgramType::Token{
                Plan::execute_token_plan(accounts, plan);
            }   
        }
        Ok(())
    }
}

fn main(){
    let mut accounts: Vec<Account> = Vec::new();
    let mut transactions: Vec<TxInstruction> = Vec::new();
    Account::create_new(&mut accounts, 1, 1000, 0, ProgramType::System);
    Account::create_new(&mut accounts, 2, 0, 0, ProgramType::Token);
    Account::create_new(&mut accounts, 3, 0, 0, ProgramType::System);
    Account::create_new(&mut accounts, 4, 0, 10000, ProgramType::Token);
    Account::print_account_state(&accounts);
    TxInstruction::create_tx_inst(&mut transactions, 1, 3, 500, ProgramType::System);
    TxInstruction::create_tx_inst(&mut transactions, 4, 2, 5000, ProgramType::Token);
    TxInstruction::print_tx_state(&transactions);
    let runtime_1 = RunTime::Runtime(&mut accounts, &transactions);
    match runtime_1{
        Ok(value)=>println!("Validation successful and execution done"),
        Err(msg)=>println!("{}",msg),
    }
    Account::print_account_state(&accounts);

}