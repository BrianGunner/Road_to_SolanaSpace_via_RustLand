#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
 enum AccountStatus {
    Active,
    Inactive,
    Frozen,
     
 }

 enum BankError{
    AccountNotFound,
    InsufficientFunds,
    AccountFrozen,
 }



struct BankAccount{
    id:u32,
    balance:i64,
    status:AccountStatus,
    
 }


 impl BankAccount {
    fn create_new_account(acc:&mut Vec<BankAccount>,id:u32){
        acc.push(BankAccount { id:id, balance: 0,status:AccountStatus::Active });
    }

    fn add_balance(accounts:&mut Vec<BankAccount>,amount:i64,id:u32)->Result<i64,BankError>{
        for acc in accounts.iter_mut(){
            if acc.id==id{
                acc.balance+=amount;
                return Ok(acc.balance);
            }
        }
        return Err(BankError::AccountNotFound);
        
    }

    fn get_balance(accounts:&Vec<BankAccount>,id:u32)->Result<i64,BankError>{
        for acc in accounts.iter(){
            if acc.id==id{
                return Ok(acc.balance);
            }
            
        }
        return Err(BankError::AccountNotFound);
    }

    fn is_active(&self)->bool{
       match self.status{
        AccountStatus::Active=>true,
        _=>false,
       }
    }

    fn check_active_status(accounts:&Vec<BankAccount>,id:u32)->Result<AccountStatus,BankError>{
        for acc in accounts.iter(){
            if acc.id==id{
                return Ok(acc.status);
            }
        }
        Err(BankError::AccountNotFound)
    }

    fn freeze(accounts:&mut Vec<BankAccount>,id:u32)->Result<(),BankError>{
        for acc in accounts.iter_mut(){
            if acc.id==id{
                acc.status=AccountStatus::Frozen;
                return Ok(());
            }
        }
        Err(BankError::AccountNotFound)
    }

    fn withdraw(accounts:&mut Vec<BankAccount>,amount:i64,id:u32)->Result<i64,BankError>{
       for acc in accounts.iter_mut(){
        if acc.id==id{
            if acc.status==AccountStatus::Frozen{
                return Err(BankError::AccountFrozen);
            
            }
            if acc.balance<amount{
                return Err(BankError::InsufficientFunds);
            }
            if acc.balance>=amount{
                acc.balance-=amount;
                return Ok(acc.balance);
            }
        }
        
    }
    return Err(BankError::AccountNotFound);
       }
        
    }
     
 


 fn main(){
    let mut accounts: Vec<BankAccount> = Vec::new();
    BankAccount::create_new_account(&mut accounts, 2);
    BankAccount::create_new_account(&mut accounts, 3);
    let amount: i64 = 999;
    let result_2 = BankAccount::add_balance(&mut accounts,amount,3);
    match result_2{
        Ok(value)=>println!("Adding {}... Balance: {}",amount,value),
        Err(BankError::AccountNotFound)=>println!("Account Not Found"),
        Err(BankError::InsufficientFunds)=>println!("Insufficient Funds"),
        Err(BankError::AccountFrozen)=>println!("Account Frozen"),
    }
    let amount_2: i64 = 999;
    let result_3 = BankAccount::add_balance(&mut accounts,amount_2,3);
    match result_3{
        Ok(value)=>println!("Adding {}... Balance: {}",amount,value),
        Err(BankError::AccountNotFound)=>println!("Account Not Found"),
        Err(BankError::InsufficientFunds)=>println!("Insufficient Funds"),
        Err(BankError::AccountFrozen)=>println!("Account Frozen"),
    }
    let output = BankAccount::get_balance(&accounts, 6);
    match output {
        Ok(value)=>println!("Checking Balance...Balance is {}",value),
        Err(BankError::AccountNotFound)=>println!("Account not found"),
        Err(BankError::InsufficientFunds)=>println!("Insufficient Funds"),
        Err(BankError::AccountFrozen)=>println!("Account Frozen"),
        
    }

    for acc in accounts.iter(){
        println!("id:{}-balance:{}",acc.id,acc.balance)
    }

    BankAccount::freeze(&mut accounts, 2);

    for acc in accounts.iter(){
        if acc.id==3{
            let status = acc.is_active();
            println!("acc.id: {} is {}",acc.id,status);
        }
    }

    match BankAccount::check_active_status(&accounts, 8){
        Ok(status)=>println!("{:?}",status),
        Err(BankError::AccountNotFound)=>println!("Account Not Found"),
        Err(BankError::InsufficientFunds)=>println!("Insufficient Funds"),
        Err(BankError::AccountFrozen)=>println!("Account Frozen"),
    }

    BankAccount::add_balance(&mut accounts,320000, 3);

    match BankAccount::withdraw(&mut accounts, 32000, 3){
        Ok(success)=>println!("Amount withdraw, new balance: {}",success),
        Err(BankError::InsufficientFunds)=>println!("Insufficient funds"),
        Err(BankError::AccountNotFound)=>println!("Account Not Found"),
        Err(BankError::AccountFrozen)=>println!("Account Frozen"),
    }
    



 }