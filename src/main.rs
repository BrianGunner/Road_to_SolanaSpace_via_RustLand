
#[derive(Debug)]
#[derive(PartialEq)]
enum ProgramType{
    System,
    Token,
    Other,
}
struct Account{
    address:u32,
    lamports:i32,
    owner:ProgramType,
}
impl Account{
    fn create_account(accounts:&mut Vec<Account>,address:u32,lamports:i32,owner:ProgramType)->Result<(),String>{
        for acc in accounts.iter(){
            if acc.address==address{
                return Err("Address already exists".to_string());
            }
        }
        accounts.push(Account{address,lamports,owner});
        Ok(())
    }
    fn print_account_state(accounts:&[Account]){
        for acc in accounts.iter(){
            println!("Address: {},Lamports: {}, Owner: {:?}", acc.address,acc.lamports,acc.owner)
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
    from_add:u32,
    to_add:u32,
    amount:i32,
    program:ProgramType,
}

impl TxInstruction{
    fn create_tx_instruction(transactions:&mut Vec<TxInstruction>,from_add:u32,to_add:u32,amount:i32,program:ProgramType){
        transactions.push(TxInstruction { from_add, to_add, amount, program })
    }
    fn print_tx_state(transactions:&[TxInstruction]){
        for txs in transactions.iter(){
            println!("From: {},To: {},Amount: {},Program: {:?}",txs.from_add,txs.to_add,txs.amount,txs.program)
        }
    }

    fn validate_txs(accounts:&[Account],tx:&TxInstruction)->Result<Plan,String>{
        if tx.program!=ProgramType::System{
            return Err("Program not System".to_string())
        }
        let from_index = Account::find_index(&accounts, tx.from_add).ok_or("from not found".to_string())?;
        let to_index = Account::find_index(&accounts, tx.to_add).ok_or("to not found".to_string())?;
        if from_index==to_index{
            return Err("From id cannot be same as to id".to_string())
        }
        if accounts[from_index].lamports<tx.amount{
            return Err("Insufficient Balance".to_string());
        }
        if tx.amount<=0{
            return Err("Transfer amount should be more than zero".to_string());
        }

        if accounts[from_index].owner!=ProgramType::System{
            return Err("From Account owner not System".to_string());
        }
        if accounts[to_index].owner!=ProgramType::System{
            return Err("To id owner not System".to_string());
        }
        Ok(Plan { from_index, to_index, amount: tx.amount })
    }

    }

struct Plan{
    from_index:usize,
    to_index:usize,
    amount:i32,
}

impl Plan{
    fn execute_plan(accounts:&mut [Account],plan_e:&Plan){
        accounts[plan_e.from_index].lamports-=plan_e.amount;
        accounts[plan_e.to_index].lamports+=plan_e.amount;
    }
    fn print_plans(plans:&[Plan]){
        for plan in plans.iter(){
            println!("From index: {},To index: {},Amount: {}",plan.from_index,plan.to_index,plan.amount)
        }
    }
}

struct RunTime;
impl RunTime{
    fn process_instructions(accounts:&mut [Account],transactions:&[TxInstruction])->Result<(),String>{
       let mut plans = Vec::new();
       for txs in transactions.iter(){
            let tx_1 = TxInstruction::validate_txs(accounts, txs);
            match tx_1{
                Ok(value)=>{
                    plans.push(value);
                },
                Err(msg)=>{
                    return Err(msg);
                }
            }
            
        }
        for plan_ in plans.iter(){
            Plan::execute_plan(accounts, plan_);
        }
        Ok(())
    }
}
fn main(){
    let mut accounts: Vec<Account> = Vec::new();
    let mut transactions: Vec<TxInstruction> = Vec::new();
    Account::create_account(&mut accounts, 1, 99, ProgramType::System);
    Account::create_account(&mut accounts, 2, 1099, ProgramType::System);
    Account::create_account(&mut accounts, 3, 99, ProgramType::System);
    Account::create_account(&mut accounts, 4, 0, ProgramType::System);
    Account::print_account_state(&accounts);
    TxInstruction::create_tx_instruction(&mut transactions, 1, 4, 55, ProgramType::System);
    TxInstruction::create_tx_instruction(&mut transactions, 2, 4, 1000, ProgramType::System);
    TxInstruction::create_tx_instruction(&mut transactions, 3, 4, 99, ProgramType::System);
    match RunTime::process_instructions(&mut accounts, &transactions){
        Ok(_)=>println!("Transactions successfully submitted"),
        Err(msg)=>println!("{}",msg),
    }
    Account::print_account_state(&accounts);
    
}