struct BankAccount{
    id:u32,
    owner:String,
    balance:i64
}

impl BankAccount{
    fn new(id:u32,owner:String,balance:i64)->BankAccount{
        BankAccount { id, owner, balance }
    }
    fn print_acc_details(&self){
        println!("Id:{},Balance:{},Name:{}",self.id,self.balance,self.owner);
    }
    fn deposit(&mut self,amount:i64){
        self.balance += amount
    }
    fn withdraw(&mut self,amount:i64){
        if self.balance>amount{
            self.balance-=amount
        }
        else{
            println!("Insufficient funds")
        }
    }
    fn is_rich(&self)->bool{
        let mut is_rich = false;
        if self.balance>5000{
            is_rich=true
        }
        return is_rich;

    }

    fn ten_k_account(name:String)->BankAccount{
        BankAccount { id: 0, owner: name, balance: 10000 }
    }
}


fn main(){
    let mut acc = BankAccount::new(2, "Stanley's".to_string(), 25000);
    acc.print_acc_details();
    acc.deposit(99);
    acc.print_acc_details();
    acc.withdraw(10000);
    acc.deposit(10000);
    acc.print_acc_details();
    println!("The account is rich? - {}",acc.is_rich());
    let mut acc2 = BankAccount::ten_k_account("Potti".to_string());
    acc2.print_acc_details();

    
}