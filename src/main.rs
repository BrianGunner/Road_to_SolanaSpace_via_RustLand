    #[derive(Debug)]
    #[derive(PartialEq)]
    enum ProgramType{
        System,
        Token,
        Other
    }

    struct Account{
        address:u32,
        lamports:i32,
        owner:ProgramType,
    }
    #[derive(Debug)]
    struct TransferInstruction{
        from_address:u32,
        to_address:u32,
        amount:i32,
        program:ProgramType,
    }
    #[derive(Debug)]
    struct Plan{
        from_index:usize,
        to_index:usize,
        amount:i32,
    }

    impl TransferInstruction {
        fn create_tx_ins(from_add:u32,to_add:u32,amount:i32,program:ProgramType)->TransferInstruction{
            TransferInstruction{from_address:from_add,to_address:to_add,amount:amount,program:program}
    }
        fn validate_tx_ins(accounts:&[Account],instruction:&TransferInstruction)->Result<Plan,String>{
            if instruction.program!=ProgramType::System{
                return Err("Cannot process Instruction as Program not System".to_string());
            }
            let from_index = Account::find_index(&accounts, instruction.from_address).ok_or("From address does not exist".to_string())?;
            let to_index = Account::find_index(&accounts, instruction.to_address).ok_or("To address does not exist".to_string())?;

            if from_index==to_index{
                return Err("From address cannot be same as to address".to_string());
            }
            if instruction.amount>accounts[from_index].lamports{
                return Err("Insufficient Balance".to_string());
            }
            if instruction.amount<=0{
                return Err("Transfer amount cannot be 0 or less".to_string());
            }
            if accounts[from_index].owner!=ProgramType::System{
                return Err("From address is not owned by System".to_string());
            }
            if accounts[to_index].owner!=ProgramType::System{
                return Err("To address is not owned by System".to_string());
            }
            Ok(Plan{from_index,to_index,amount:instruction.amount})
        }
    }

        

    impl Account{
        fn create_account(accounts:&mut Vec<Account>,address:u32,owner:ProgramType)->Result<(),String>{
            for acc in accounts.iter(){
                if acc.address==address{
                    return Err("Address already exists".to_string())
                }
            }
            accounts.push(Account { address, lamports: 100, owner});
            return Ok(());
        }

        fn print_state(accounts:&[Account]){
            for acc in accounts.iter(){
                println!("Address: {}, Lamports: {}, owner: {:?}",acc.address,acc.lamports,acc.owner)
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
        


    fn main(){
        let mut accounts: Vec<Account> = Vec::new();  
        Account::create_account(&mut accounts, 1, ProgramType::System);
        Account::create_account(&mut accounts, 2, ProgramType::Other);
        Account::create_account(&mut accounts, 2, ProgramType::System);
        Account::create_account(&mut accounts, 4, ProgramType::System);
        Account::create_account(&mut accounts, 5, ProgramType::System);
        Account::print_state(&accounts);
        let ins_1 = TransferInstruction::create_tx_ins(1, 2, 100,ProgramType::System);
        let ins_2 = TransferInstruction::create_tx_ins(4, 5, 34,ProgramType::System);
        let validate_ins_2 = TransferInstruction::validate_tx_ins(&accounts,&ins_2);
        match validate_ins_2{
            Ok(value)=>println!("{:?}",value),
            Err(msg)=>println!("{}",msg),
        }
    }
