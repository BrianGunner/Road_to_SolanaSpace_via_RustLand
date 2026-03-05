#[derive(Debug)]
enum AccountStatus{
    Active,
    Inactive,
    Frozen,
}

struct BankAccount{
    id:u32,
    owner:String,
    balance:i64,
    status:AccountStatus,
}

impl BankAccount{
    fn new(id:u32,owner:String)->BankAccount{
        BankAccount { id:id, owner:owner, balance:0, status:AccountStatus::Active}
    }

    fn print_details(&self){
        println!("Name: {}, id: {}, balance: {}, status: {:?}",self.owner,self.id,self.balance,self.status);
    }

    fn is_active(&self)->bool{
        match self.status{
            AccountStatus::Active=>true,
            _=>false,
        }
    }

    fn freeze(&mut self){
        self.status=AccountStatus::Frozen
    }

    fn unfreeze(&mut self){
        self.status = AccountStatus::Active
    }

    fn status_message(&self)->&str{
        match self.status{
            AccountStatus::Active=>"Account is active",
            AccountStatus::Frozen=>"Account is frozen",
            _=>"Account is inactive",
        }
    }

    fn deposit(&mut self,amount:i64){
        match self.status{
            AccountStatus::Frozen=>{
                println!("Account Frozen cannot deposit")
            }
            _=>self.balance+=amount
            
        }
    }

    fn withdraw(&mut self,amount:i64){
        match self.status {
            AccountStatus::Active=>if self.balance>amount{
                self.balance-=amount
            }
            else{
                println!("Insufficient Funds")
            }
            _=>println!("Cannot withdraw"),
            
        }
    }
}

fn main(){
    let mut acc1 = BankAccount::new(1, "Bhargav".to_string());
    acc1.print_details();
    println!("Account status: {}",acc1.is_active());
    acc1.freeze();
    println!("Account status: {}",acc1.is_active());
    println!("{}",acc1.status_message());
    acc1.unfreeze();
    acc1.print_details();
    acc1.deposit(25000);
    acc1.print_details();
    acc1.withdraw(26000);

}