
struct PlayerAccount{
    id:u32,
    balance:i64,
}

impl Describable for PlayerAccount{
    fn describe (&self) {
        println!("id: {}, balance: {}",self.id,self.balance);
    }
}

impl Transferable for PlayerAccount{
    fn deposit(&mut self,amount:i64) {
        self.balance += amount
    }
    fn withdraw(&mut self,amount:i64)->Result<i64,String> {
        if self.balance>=amount{
            self.balance-=amount;
            return Ok(self.balance);
        }
        Err("Insufficient Funds".to_string())
    }
    
}

struct TreasuryAccount{
    id:u32,
    balance:i64,
}

impl Describable for TreasuryAccount{
    fn describe (&self) {
        println!("id: {}, balance: {}",self.id,self.balance);
    }
}

impl Transferable for TreasuryAccount{
    fn deposit(&mut self,amount:i64) {
        self.balance += amount
    }
    fn withdraw(&mut self,amount:i64)->Result<i64,String> {
        if self.balance>=amount{
            self.balance-=amount;
            return Ok(self.balance)
        }
        Err("Insufficient Funds".to_string())
    }
}

struct LotteryAccount{
    id:u32,
    balance:i64,
}

impl Describable for LotteryAccount {
    fn describe (&self) {
        println!("id: {}, balance: {}",self.id,self.balance);
    }
    
}

trait Describable {
    fn describe (&self);
}

trait Transferable{
    fn deposit(&mut self,amount:i64);
    fn withdraw(&mut self,amount:i64)->Result<i64,String>;
}



fn print_info(acc:&impl Describable){
    acc.describe();
}

fn fund_account(acc: &mut impl Transferable,amount:i64){
    acc.deposit(amount);
    println!("Account funded with amount: {}",amount);

}

fn check_withdraw(acc:&mut impl Transferable,amount:i64){
    match acc.withdraw(amount){
        Ok(value)=>println!("Amount withdrawn, current balance: {}",value),
        Err(msg)=>println!("{}",msg),
    }
   
}





fn main(){
    let mut playeraccount = PlayerAccount{id:1,balance:100};
    let mut treasuryaccount = TreasuryAccount{id:1,balance:25000};
    let mut lotteryaccount = LotteryAccount{id:1,balance:500};
   
   playeraccount.describe();
   treasuryaccount.describe();
   lotteryaccount.describe();
   print_info(&playeraccount);
   print_info(&treasuryaccount);
   print_info(&lotteryaccount);
   fund_account(&mut playeraccount, 500);
   print_info(&playeraccount);
   fund_account(&mut treasuryaccount, 500);
   print_info(&treasuryaccount);
   check_withdraw(&mut playeraccount, 500);
   check_withdraw(&mut treasuryaccount, 50000);


}