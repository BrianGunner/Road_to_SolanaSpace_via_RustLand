struct Account{
    id:u32,
    balance:i32,
}

impl Account {
    fn new(accounts:&mut Vec<Account>,id:u32){
        accounts.push(Account { id: id, balance: 0 });
    }   
    fn add_balance(accounts:&mut Vec<Account>,id:u32,amount:i32){
        for acc in accounts.iter_mut(){
            if acc.id==id{
                acc.balance+=amount
            }
        }
    } 
    fn print_accounts_state(accounts:&Vec<Account>){
        println!("======🤑 Account State 🤑======");
        
        for acc in accounts.iter(){
            println!("id: {}, balance: {}",acc.id,acc.balance)
        }

        println!("======🤑 Account State 🤑======");
    }
}

fn main(){
    let mut accounts: Vec<Account> = Vec::new();
    Account::new(&mut accounts, 1);
    Account::add_balance(&mut accounts, 1, 500);
    Account::print_accounts_state(&accounts);
}