struct BankAccount{
    id:u32,
    balance:i64,
}

struct SavingsAccount{
    id:u32,
    balance:i64,
}

struct LoanAccount{
    id:u32,
    balance:i64,
    owner:String,
}

trait Describable {
    fn describe(&self);
    
}

trait Transferable {

    fn transfer(&mut self,amount:i64)->Result<i64,String>;
    
}

impl Describable for BankAccount {

    fn describe(&self) {
        println!("Balance in Bank Account is : {}",self.balance);
    }
    
}

impl Transferable for BankAccount{
    fn transfer(&mut self,amount:i64)->Result<i64,String> {
        if self.balance>=amount{
            self.balance-=amount;
            return Ok(self.balance);
        }
        Err("Not enough Balance".to_string())
    }
}

impl Describable for SavingsAccount {

    fn describe(&self) {
        println!("Balance in Savings Account is: {}",self.balance);
    }
    
}

impl Transferable for SavingsAccount {

    fn transfer(&mut self,amount:i64)->Result<i64,String> {
        if self.balance>=amount{
            self.balance-=amount;
            return Ok(self.balance);
        }
        Err("Not enough balance".to_string())
    }
    
}

impl Describable for LoanAccount{
    fn describe(&self) {
        println!("Balance in loan Account is: {}",self.balance);
    }
}




fn main(){
    let mut bank_account = BankAccount{id:1,balance:2000};
    let mut savings_account =  SavingsAccount{id:1,balance:999};
    let mut loan_account = LoanAccount{id:1,balance:787,owner:"Bhagav".to_string()};
    bank_account.describe();
    savings_account.describe();
    loan_account.describe();
    match bank_account.transfer(9999){
        Ok(value)=>println!("Amount deducted from BankAccount, updated balance is: {}",value),
        Err(msg)=>println!("{}",msg),
    }
    match savings_account.transfer(9999){
        Ok(value)=>println!("Amount deducted from SavingsAccount, updated balance is: {}",value),
        Err(msg)=>println!("{}",msg),
    }
    
    

}