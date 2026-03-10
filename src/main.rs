struct PlayerAccount{
    id:u32,
    balance:i64,
}

struct TreasuryAccount{
    id:u32,
    balance:i64,
}

struct LotteryPool{
    id:u32,
    balance:i64,
}

trait Describable {
    fn describe (&self);
    
}

trait Transferable{
    fn deposit(&mut self,amount:i64);
    fn withdraw(&mut self,amount:i64)->Result<i64,String>;
}

trait HasBalance {
    fn get_balance(&self)->i64;
    fn is_funded(&self)->bool;
    
}

impl Describable for PlayerAccount{
    fn describe(&self){
        println!("Player id: {}, Player balance: {}",self.id,self.balance)
    }
}

impl Transferable for PlayerAccount{
    fn deposit(&mut self,amount:i64) {
        self.balance+=amount
    }
    fn withdraw(&mut self,amount:i64)->Result<i64,String> {
        if self.balance>=
        amount{
            self.balance-=amount;

            return Ok(self.balance);
        }
        Err("Insufficient Balance".to_string())

    }
}

impl HasBalance for PlayerAccount {

    fn get_balance(&self)->i64 {
        self.balance
    }
    fn is_funded(&self)->bool {
        self.balance>0
    }
    
}

impl Describable for TreasuryAccount{
    fn describe(&self){
        println!("Treasury id: {}, Treasury balance: {}",self.id,self.balance)
    }
}

impl Transferable for TreasuryAccount{
    fn deposit(&mut self,amount:i64) {
        self.balance+=amount

    }

    fn withdraw(&mut self,amount:i64)->Result<i64,String> {
        if self.balance>amount{
            self.balance-=amount;
            return Ok(self.balance);
        }
        Err("Insufficient Balance".to_string()
    )
    }
}

impl HasBalance for TreasuryAccount{
    fn get_balance(&self)->i64 {
        self.balance
    }
    fn is_funded(&self)->bool {
        self.balance>0
    }
}

impl Describable for LotteryPool {
    fn describe(&self){
        println!("Lottery id: {}, Lottery balance: {}",self.id,self.balance)
    }
    
}

impl HasBalance for LotteryPool{
    fn get_balance(&self)->i64 {
        self.balance
    }
    fn is_funded(&self)->bool {
        self.balance>0

    }
}

fn print_balance(acc:&impl HasBalance){
    println!("Balance is: {}",acc.get_balance());
    println!("Is funded: {}",acc.is_funded());
}

fn main(){
    let mut playeraccount = PlayerAccount{id:1,balance:999};
    let mut treasuryaccount = TreasuryAccount{id:1,balance:25000};
    let mut lotterypool = LotteryPool{id:1,balance:300};

    playeraccount.deposit(999);
    
    match playeraccount.withdraw(1998){
        Ok(value)=>println!("Withdraw successful, avaliable balance: {}",value),
        Err(msg)=>println!("{}",msg),

    }

    treasuryaccount.deposit(999);

    match treasuryaccount.withdraw(999){
        Ok(value)=>println!("Withdrawl successful, available balance: {}",value),
        Err(msg)=>println!("{}",msg),
    }

    
    playeraccount.describe();
    treasuryaccount.describe();
    lotterypool.describe();

    println!("Balance: {} and is funded: {}",playeraccount.get_balance(),playeraccount.is_funded());
    println!("Lottery pool is funded: {}",lotterypool.is_funded());
    println!("Treasury Balance: {}",treasuryaccount.balance);

    print_balance(&treasuryaccount);




   
    

}