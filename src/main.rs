


#[derive(PartialEq)]
#[derive(Debug)]
enum ProgramType{
    SystemProgram,
    TokenProgram,
}

struct Account{
    address:u32,
    lamports:u64,
    data:Vec<u8>,
    owner:ProgramType,
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
    fn find_index(accounts:&[Account],address:u32)->Result<usize,String>{
        for (index,acc) in accounts.iter().enumerate(){
            if acc.address == address{
                return Ok(index)
            }
        }
        Err("Could not find index".to_string())
    }
    fn print_state(accounts:&[Account]){
        println!("🤑+++++++++++++++++🤑");
        for acc in accounts.iter(){
            if acc.owner == ProgramType::SystemProgram{
                println!("Address: {}, Lamports Balance: {}",acc.address,acc.lamports)
            }
            else{
                let mut bytes = [0u8;8];    
                if acc.data.len()>=8{
                bytes.copy_from_slice(&acc.data[0..8]);
                let balance = u64::from_le_bytes(bytes);
                println!("Address: {}, Token Balance: {}",acc.address,balance)
                }
                else{
                    println!("Address: {}, Token Balance: NA",acc.address)
                }
            }
        
    }
}
}
#[derive(Debug)]
struct Instruction{
    from_address:u32,
    to_address:u32,
    amount:u64,
    program:ProgramType,
}
struct Plan{
    from_index:usize,
    to_index:usize,
    amount:u64,
    program:ProgramType,
}



struct SystemProgram;
impl SystemProgram{
    fn validate_instruction(accounts:&[Account],instruction:&Instruction)->Result<Plan,String>{
        if instruction.program!=ProgramType::SystemProgram{
            return Err("Program Type not System".to_string());
        }
        let from_index_instruction = Account::find_index(&accounts, instruction.from_address)?;
        let to_index_instruction = Account::find_index(&accounts, instruction.to_address)?;

        if from_index_instruction==to_index_instruction{
            return Err("From and two addresses cannot be the same".to_string());
        }
        if accounts[from_index_instruction].lamports<instruction.amount{
            return Err("Insufficient Balance".to_string())
        }
        if accounts[from_index_instruction].owner!=ProgramType::SystemProgram{
            return Err("From Account owner not System".to_string())
        }
        if accounts[to_index_instruction].owner!=ProgramType::SystemProgram{
            return Err("To Account owner not System".to_string())
        }
        if instruction.amount==0{
            return Err("Transfer amount cannot be Zero".to_string());
        }
        Ok(Plan { from_index: from_index_instruction, to_index: to_index_instruction, amount: instruction.amount,program:ProgramType::SystemProgram })
    

    }
    fn execute_plan(accounts:&mut [Account],plan:&Plan){
        accounts[plan.from_index].lamports-=plan.amount;
        accounts[plan.to_index].lamports+=plan.amount;
    }
}
struct TokenProgram;
impl TokenProgram{
    fn read_balance(account:&Account)->u64{
        let mut bytes = [0u8;8];
        bytes.copy_from_slice(&account.data[0..8]);
        u64::from_le_bytes(bytes)
    }
    fn write_balance(account:&mut Account,amount:u64){
        let amount_bytes = amount.to_le_bytes().to_vec();
        account.data=amount_bytes
    }
    fn validate_instruction(accounts:&[Account],instruction:&Instruction)->Result<Plan,String>{
        if instruction.program!=ProgramType::TokenProgram{
            return Err("Program Type not Token".to_string());
        }
        let from_index_instruction = Account::find_index(&accounts, instruction.from_address)?;
        let to_index_instruction = Account::find_index(&accounts, instruction.to_address)?;
        if from_index_instruction==to_index_instruction{
            return Err("From and to addresses cannot be the same".to_string());
        }
        if accounts[from_index_instruction].owner!=ProgramType::TokenProgram{
            return Err("From id owner not Token".to_string());
        }
        if accounts[to_index_instruction].owner!=ProgramType::TokenProgram{
            return Err("To id owner not Token".to_string());
        }
        let mut data_store = [0u8;8];
        data_store.copy_from_slice(&accounts[from_index_instruction].data[0..8]);
        let from_token_value = u64::from_le_bytes(data_store);

        if from_token_value<instruction.amount{
            return Err("Insufficient Balance".to_string());
        }
        if instruction.amount==0{
            return Err("Transfer amount cannot be Zero".to_string());
        }
        Ok(Plan{from_index:from_index_instruction,to_index:to_index_instruction,amount:instruction.amount,program:ProgramType::TokenProgram})
    

    }

    fn execute_plan(accounts:&mut [Account],plan:&Plan){
        let from_token_balance = TokenProgram::read_balance(&accounts[plan.from_index]);
        let to_token_balance = TokenProgram::read_balance(&accounts[plan.to_index]);
        let updated_from_balance = from_token_balance-plan.amount;
        let updated_to_balance = to_token_balance+plan.amount;
        TokenProgram::write_balance(&mut accounts[plan.from_index], updated_from_balance);
        TokenProgram::write_balance(&mut accounts[plan.to_index], updated_to_balance);

    }

    
}

struct RunTime;

impl RunTime{
    
    fn validate_transactions(accounts:&[Account],instruction:&Instruction)->Result<Plan,String>{
        match instruction.program{
            ProgramType::SystemProgram=>{
            let result = SystemProgram::validate_instruction(&accounts, instruction)?;
            Ok(result)},
            ProgramType::TokenProgram=>{
                let result = TokenProgram::validate_instruction(&accounts, instruction)?;
                Ok(result)
            },
        }     
    }
    fn process_transactions(accounts:&mut [Account],tx:&Transaction)->Result<(),String>{
        let mut plan_vec: Vec<Plan> = Vec::new();
        for t in tx.instruction_vec.iter(){
            let result = RunTime::validate_transactions(&accounts, t);
            match result{
                Ok(value)=>plan_vec.push(value),
                Err(msg)=>return Err(msg)
            }
        }
        for plan in plan_vec.iter(){
            match plan.program{
                ProgramType::SystemProgram=>SystemProgram::execute_plan(accounts, plan),
                ProgramType::TokenProgram=>TokenProgram::execute_plan(accounts, plan),
            }
        }
        Ok(())
            
        }
    }



    
    
#[derive(Debug)]
struct Transaction{
    instruction_vec:Vec<Instruction>
}

impl Transaction {
    fn add_transaction(&mut self,from_address:u32,to_address:u32,amount:u64,program:ProgramType){
        let instruction_new = Instruction{from_address,to_address,amount,program};
        self.instruction_vec.push(instruction_new);
    }
    fn print_tx_set(&self){
        println!("{:?}",self.instruction_vec)
    }
}







fn main(){

    let mut accounts: Vec<Account> = Vec::new();


    match Account::create_account(&mut accounts, 1, 1000, vec![], ProgramType::SystemProgram){
        Ok(_)=>println!("Account created"),
        Err(msg)=>println!("{}",msg),
    }
    match Account::create_account(&mut accounts, 2, 1000, vec![], ProgramType::SystemProgram){
       Ok(_)=>println!("Account created"),
        Err(msg)=>println!("{}",msg), 
    }
    
    
    let token = 10000_u64;
    let bytes = token.to_le_bytes().to_vec();
    let token_2 = 0_u64;
    let bytes_2 = token_2.to_le_bytes().to_vec();
    Account::create_account(&mut accounts, 3, 0, bytes, ProgramType::TokenProgram);
    Account::create_account(&mut accounts, 4, 0, bytes_2, ProgramType::TokenProgram);
    
    Account::print_state(&accounts);

    let mut tx_1 = Transaction{instruction_vec:Vec::new()};
    tx_1.add_transaction(1, 2, 100, ProgramType::SystemProgram);
    tx_1.add_transaction(3, 3, 599, ProgramType::TokenProgram);
    match RunTime::process_transactions(&mut accounts, &tx_1){
        Ok(value)=>println!("Success"),
        Err(msg)=>println!("{}",msg),
    }

    Account::print_state(&accounts);



}