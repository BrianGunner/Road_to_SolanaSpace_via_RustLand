

#[derive(Debug, PartialEq, Clone)]
enum ProgramType{
    System,
    Token,
}
#[derive(Debug, Clone)]
struct Account{
    address:u32,
    lamports:u64,
    data:Vec<u8>,
    owner:ProgramType,
}
impl Account{
    fn print_state(accounts:&[Account]){
        for acc in accounts.iter(){
            println!("Address: {}, Lamports: {}, data: {:?}, owner: {:?}",acc.address,acc.lamports,acc.data,acc.owner)
        }
    }
    fn find_index(accounts:&[Account],address:u32)->Result<usize,String>{
        for (index,acc) in accounts.iter().enumerate(){
            if acc.address==address{
                return Ok(index);
            }
        }
        Err("Address does not exist".to_string())
    }
}
#[derive(Debug)]
struct Instruction{
    from_address:u32,
    to_address:u32,
    amount:u64,
    program:ProgramType,
    signer_address:u32,
}
#[derive(Debug)]
struct Plan{
    from_index:usize,
    to_index:usize,
    amount:u64,
    program:ProgramType,
}



struct SystemProgram;
impl SystemProgram{
    
    fn validate_instruction(accounts:&[Account],instruction:&Instruction)->Result<Plan,String>{
        if instruction.program!=ProgramType::System{
            return Err("Program not System".to_string())
        }
        let from_index = Account::find_index(&accounts, instruction.from_address)?;
        let to_index = Account::find_index(&accounts, instruction.to_address)?;
        if from_index==to_index{
            return Err("From and To addresses cannot be same".to_string());
        }
        if accounts[from_index].owner!=ProgramType::System{
            return Err("From address owner not System".to_string());
        }
        if instruction.signer_address!=instruction.from_address{
            return Err("From address did not sign tx".to_string());
        }
        if accounts[to_index].owner!=ProgramType::System{
            return Err("To address owner not System".to_string());
        }
        if accounts[from_index].lamports<instruction.amount{
            return Err("Not enough balance".to_string());
        }        
        Ok(Plan{from_index:from_index,to_index:to_index,amount:instruction.amount,program:ProgramType::System})
    }
    fn execute_plan(accounts:&mut[Account],plan:&Plan){
        accounts[plan.from_index].lamports-=plan.amount;
        accounts[plan.to_index].lamports+=plan.amount;
    }
}

struct TokenProgram;
    impl TokenProgram{
    fn read_balance(account:&Account)->u64{
        let account_vec = &account.data;
        let mut balance = [0u8;8];
        balance.copy_from_slice(&account_vec[0..8]);
        let amount = u64::from_le_bytes(balance);
        return amount;
    }
    fn write_balance(account:&mut Account,amount:u64){
        let amount_bytes = amount.to_le_bytes().to_vec();
        account.data = amount_bytes;

    }
    fn validate_instruction(accounts:&[Account],instruction:&Instruction)->Result<Plan,String>{
        if instruction.program!=ProgramType::Token{
            return Err("Program not Token".to_string());
        }
        let from_index = Account::find_index(&accounts,instruction.from_address)?;
        let to_index = Account::find_index(&accounts, instruction.to_address)?;

        if from_index==to_index{
            return Err("From address cannot be same as to address".to_string());
        }
        if accounts[from_index].owner!=ProgramType::Token{
            return Err("From address not Token".to_string());
        }
        if instruction.signer_address!=instruction.from_address{
            return Err("From address did not sign tx".to_string());
        }
        if accounts[to_index].owner!=ProgramType::Token{
            return Err("To address not Token".to_string());
        }
        if instruction.amount==0{
            return Err("Transfer amount cannot be Zero".to_string());
        }
        let from_balance = TokenProgram::read_balance(&accounts[from_index]);
        
        if from_balance<instruction.amount{
            return Err("Not enough balance".to_string());
        }
        Ok(Plan{from_index:from_index,to_index:to_index,amount:instruction.amount,program:ProgramType::Token})

    }

    fn execute_plan(accounts:&mut [Account],plan:&Plan){
        let current_from_balance = TokenProgram::read_balance(&accounts[plan.from_index]);
        let current_to_balance = TokenProgram::read_balance(&accounts[plan.to_index]);
        let updated_from_balance = current_from_balance - plan.amount;
        let updated_to_balance = current_to_balance+plan.amount;
        TokenProgram::write_balance(&mut accounts[plan.from_index], updated_from_balance);
        TokenProgram::write_balance(&mut accounts[plan.to_index], updated_to_balance);
    }
    fn serialize(tokendata:&TokenAccountData)->Vec<u8>{
        let mut data: Vec<u8> = Vec::new();
        let balance_bytes = tokendata.balance.to_le_bytes();
        for bytes in balance_bytes.iter(){
            data.push(*bytes);
        }
        if tokendata.frozen==true{
            data.push(1);
        }
        else{
            data.push(0);
        }
        return data;

  

    }
    
}

#[derive(Debug)]
struct TokenAccountData{
    balance:u64,
    frozen:bool,
}
struct Transaction{
    instructions:Vec<Instruction>,  
}
struct RunTime;
impl RunTime{

    fn process_transaction(accounts:&mut [Account],transaction:&Transaction)->Result<(),String>{
        let mut plan_vector: Vec<Plan> = Vec::new();
        for tx in transaction.instructions.iter(){
            match tx.program{
                ProgramType::System=>{
                    let plan_system = SystemProgram::validate_instruction(accounts, tx);
                    match plan_system{
                        Ok(value)=>plan_vector.push(value),
                        Err(msg)=>return Err(msg),
                    }
                }
                ProgramType::Token=>{
                    let plan_token = TokenProgram::validate_instruction(accounts, tx);
                    match plan_token{
                        Ok(value)=>plan_vector.push(value),
                        Err(msg)=>return Err(msg),
                    }
                }
            }
        }
        for plan_e in plan_vector.iter(){
            match plan_e.program{
                ProgramType::System=>SystemProgram::execute_plan(accounts, plan_e),
                ProgramType::Token=>TokenProgram::execute_plan(accounts, plan_e),
            }
        }
        Ok(())
    }
    

    }
    



fn main(){
    let mut accounts: Vec<Account> = Vec::new();
    
    let account_1 = Account{address:1,lamports:9999,data:vec![],owner:ProgramType::System};
    accounts.push(account_1);
 
    let account_2 = Account{address:2,lamports:500,data:vec![],owner:ProgramType::System};
    accounts.push(account_2);
    Account::print_state(&accounts);
    match Account::find_index(&accounts, 2){
        Ok(value)=>println!("Found Account index {}",value),
        Err(msg)=>println!("{}",msg),
    }
    let tokens = 100000_u64;
    let tokens_2 = 0_u64;
    let tokens_2_bytes = tokens_2.to_le_bytes().to_vec();
    let tokens_bytes = tokens.to_le_bytes().to_vec();
    let account_3 = Account{address:3,lamports:0,data:tokens_bytes,owner:ProgramType::Token};
    let account_4 = Account{address:4,lamports:0,data:tokens_2_bytes,owner:ProgramType::Token};
    accounts.push(account_3);
    accounts.push(account_4);
    let inst_1 = Instruction{from_address:1,to_address:2,amount:110,program:ProgramType::System,signer_address:1};
    let inst_2 = Instruction{from_address:3,to_address:4,amount:500,program:ProgramType::Token,signer_address:3};
    let tx_1 = Transaction{instructions:vec![inst_1,inst_2]};
    match RunTime::process_transaction(&mut accounts, &tx_1){
        Ok(_)=>println!("Tx successful"),
        Err(msg)=>println!("{:?}",msg),
    }
    Account::print_state(&accounts);
    

    let tad_1 = TokenAccountData{balance:10000,frozen:true};
    let sample = TokenProgram::serialize(&tad_1);
    println!("{:?}",sample)
    

    
}