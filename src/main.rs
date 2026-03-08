

struct BankAccount{
    id:u32,
    balance:i64,
 }

 impl BankAccount {
    fn create_new_account(acc:&mut Vec<BankAccount>,id:u32){
        acc.push(BankAccount { id:id, balance: 0 });
    }

    fn add_balance(accounts:&mut Vec<BankAccount>,amount:i64,id:u32)->Result<i64,String>{
        for acc in accounts.iter_mut(){
            if acc.id==id{
                acc.balance+=amount;
                return Ok(acc.balance);
            }
        }
        return Err("Could not add balance, id not found".to_string());
        
    }

    fn get_balance(accounts:&Vec<BankAccount>,id:u32)->Result<i64,String>{
        for acc in accounts.iter(){
            if acc.id==id{
                return Ok(acc.balance);
            }
            
        }
        return Err("Could not find account id".to_string());
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
        Err(msg)=>println!("{}",msg),
    }
    let amount_2: i64 = 999;
    let result_3 = BankAccount::add_balance(&mut accounts,amount_2,3);
    match result_3{
        Ok(value)=>println!("Adding {}... Balance: {}",amount,value),
        Err(msg)=>println!("{}",msg),
    }
    let output = BankAccount::get_balance(&accounts, 3);
    match output {
        Ok(value)=>println!("Checking Balance...Balance is {}",value),
        Err(message)=>println!("{}",message),
        
    }

    for acc in accounts.iter(){
        println!("id:{}-balance:{}",acc.id,acc.balance)
    }



 }