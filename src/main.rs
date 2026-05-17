    #[derive(Debug)]
    struct Wallet{
        id:u32,
        owner:String,
        balance:u64,
    }

    impl Wallet{
        fn new(id:u32,owner:String,open_balance:u64)->Result<Wallet,String>{
            if open_balance==0{
                return Err("Balance cannot be zero".to_string())
            }
            Ok(Wallet { id, owner, balance: open_balance })
        }

        fn deposit(&mut self,amount:u64){
            self.balance+=amount
        }

        fn withdraw(&mut self,amount:u64)->Result<(),String>{
            if self.balance<amount{
                return Err("Insufficient Balance".to_string())
            }
            self.balance-=amount;
            Ok(())
        }

        fn print_wallet(&self){
            println!("id: {},name: {}, balance: {}",self.id,self.owner,self.balance)
        }
    }

    fn main()->Result<(),String>{
    let mut wallet_1 =  Wallet::new(1, "Bhargav".to_string(), 99)?;
    wallet_1.deposit(100);
    wallet_1.print_wallet();
    wallet_1.withdraw(100);
    wallet_1.print_wallet();
    let test_case = wallet_1.withdraw(999);
    match test_case {
        Ok(value)=>println!("Withdraw successful"),
        Err(msg)=>println!("{}",msg),
        
    }
    Ok(())
        
    }
        
        