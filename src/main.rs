



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
    data:Vec<u8>,
    owner:ProgramType
}

impl Account{
    fn create_account(accounts:&mut Vec<Account>,address:u32,lamports:u64,data:Vec<u8>,owner:ProgramType)->Result<(),String>{
        for acc in accounts.iter(){
            if acc.address==address{
                return Err("Address already exists".to_string())
            }
        }
        accounts.push(Account { address, lamports, data, owner });
        Ok(())
    }
    fn print_state(accounts:&[Account]){
        println!("🤑+++++++++++++++++++++🤑");
        for acc in accounts.iter(){
            println!("Address: {} Lamports: {} Data: {:?} Owner: {:?}",acc.address,acc.lamports,acc.data,acc.owner)
        }
    }
    fn find_index(accounts:&[Account],address:u32)->Result<usize,String>{
        for (index,acc) in accounts.iter().enumerate(){
            if acc.address == address{
                return Ok(index)
            }
        }
        Err("Could not find index".to_string())
    }
}

struct SystemProgram;
impl SystemProgram{
    fn read_lamports(account:&Account)->u64{
        account.lamports     
}
    fn write_lamports(account:&mut Account,amount:u64){
        account.lamports = amount

    }
    fn validate_transfer(accounts:&[Account],from_index:usize,to_index:usize,amount:u64)->Result<Plan,String>{
        if from_index==to_index{
            return Err("From index cannot be same as to index".to_string())
        }
        if accounts[from_index].owner!=ProgramType::System{
            return Err("From Owner not system".to_string());
        }
        if accounts[to_index].owner!=ProgramType::System{
            return Err("To Owner not system".to_string());
        }
        if SystemProgram::read_lamports(&accounts[from_index])<amount{
            return Err("Insufficient Balance".to_string());
        }
        return Ok(Plan{from_index,to_index,amount});
    }
    fn execute_transfer(accounts:&mut [Account],plan:&Plan){
        let current_from_balance = SystemProgram::read_lamports(&accounts[plan.from_index]);
        let current_to_balance = SystemProgram::read_lamports(&accounts[plan.to_index]);
        let updated_from_balance = current_from_balance-plan.amount;
        let updated_to_balance = current_to_balance+plan.amount;
        SystemProgram::write_lamports(&mut accounts[plan.from_index], updated_from_balance);
        SystemProgram::write_lamports(&mut accounts[plan.to_index], updated_to_balance);
    }

}

struct TokenProgram;

impl TokenProgram{
    fn read_token_balance(account:&Account)->u64{
        let mut bytes = [0u8;8];
        bytes.copy_from_slice(&account.data[0..8]);
        u64::from_le_bytes(bytes)
        
    }
    fn write_token_balance(account:&mut Account,amount:u64){
        account.data = amount.to_le_bytes().to_vec();
    }

    fn validate_transfer(accounts:&[Account],from_index:usize,to_index:usize,amount:u64)->Result<Plan,String>{
        if from_index==to_index{
            return Err("From index cannot be same as to index".to_string())
        }
        if accounts[from_index].owner!=ProgramType::Token{
            return Err("From Owner not token".to_string());
        }
        if accounts[to_index].owner!=ProgramType::Token{
            return Err("To Owner not token".to_string());
        }
        if TokenProgram::read_token_balance(&accounts[from_index])<amount{
            return Err("Insufficient Balance".to_string());
        }
        return Ok(Plan{from_index,to_index,amount});
    }

    fn execute_transfer(accounts:&mut [Account],plan:&Plan){
        let from_balance_current = TokenProgram::read_token_balance(& accounts[plan.from_index]);
        let to_balance_current = TokenProgram::read_token_balance(& accounts[plan.to_index]);
        let updated_from_balance = from_balance_current-plan.amount;
        let update_to_balance = to_balance_current+plan.amount;
        TokenProgram::write_token_balance(&mut accounts[plan.from_index], updated_from_balance);
        TokenProgram::write_token_balance(&mut accounts[plan.to_index], update_to_balance);
    }

}
struct Plan{
    from_index:usize,
    to_index:usize,
    amount:u64,
}

struct Instruction{
    program:ProgramType,
    from_address:u32,
    to_address:u32,
    amount:u64,
}

struct RunTime;
impl RunTime{
    fn validate_instruction(accounts:&mut [Account],instruction:&Instruction)->Result<Plan,String>{
        let from_index_ins = Account::find_index(&accounts, instruction.from_address)?;
        let to_index_ins = Account::find_index(&accounts, instruction.to_address)?;
        if instruction.program==ProgramType::System{
            let plan = SystemProgram::validate_transfer(&accounts, from_index_ins, to_index_ins, instruction.amount)?;
            return Ok(plan);
            
        }
        if instruction.program==ProgramType::Token{
            let plan = TokenProgram::validate_transfer(&accounts, from_index_ins, to_index_ins,instruction.amount)?;
            return Ok(plan);
            
        }
        
        Err("Program Type Other".to_string())
    
    }
    fn process_instruction(accounts: &mut [Account], instruction: &Instruction) -> Result<(), String> {
    let from_index = Account::find_index(accounts, instruction.from_address)?;
    let to_index = Account::find_index(accounts, instruction.to_address)?;
    
    let plan = match instruction.program {
        ProgramType::System => SystemProgram::validate_transfer(accounts, from_index, to_index, instruction.amount)?,
        ProgramType::Token => TokenProgram::validate_transfer(accounts, from_index, to_index, instruction.amount)?,
        ProgramType::Others => return Err("Unsupported program".to_string()),
    };
    
    match instruction.program {
        ProgramType::System => SystemProgram::execute_transfer(accounts, &plan),
        ProgramType::Token => TokenProgram::execute_transfer(accounts, &plan),
        ProgramType::Others => return Err("Unsupported program".to_string()),
    };
    
    Ok(())

        
        
    }
}



fn main(){
    let mut accounts: Vec<Account> = Vec::new();
    let tokens = 10000_u64;
    let mut token_bytes = tokens.to_le_bytes().to_vec();

    Account::create_account(&mut accounts, 1, 0, token_bytes, ProgramType::Token);
    Account::create_account(&mut accounts, 2, 1000, vec![], ProgramType::System);
    Account::create_account(&mut accounts, 3, 1000, vec![], ProgramType::System);
    Account::create_account(&mut accounts, 4, 0, vec![], ProgramType::Token);

    let ins_1 = Instruction{program:ProgramType::System,from_address:2,to_address:3,amount:100};
    
    

}
