#[derive(Debug)]
struct Account{
    id:u32,
    balance:i32,
}

fn fetch_index(accounts:& Vec<Account>,id:u32)->Option<usize>{
    for (index,acc) in accounts.iter().enumerate(){
        if acc.id == id{
            return Some(index);
        }
    }
    None
}

fn modify(accounts:&mut Vec<Account>,id:u32)->Result<(),String>{
    let index_found = fetch_index(accounts, id).ok_or("Could not find index")?;
    accounts[index_found].balance += 11;
    return Ok(());
}

impl Account {

    fn print_state(accounts:& Vec<Account>){
        for acc in accounts.iter(){
            println!("{}",acc.balance)
        }
    }
    
}



fn main(){
    let mut accounts = vec![
        Account{id:1,balance:9999},
        Account{id:2,balance:0},
        Account{id:3,balance:8888},
    ];

    let modify_account = modify(&mut accounts, 9);
    match modify_account{
        Ok(())=>println!("Success"),
        Err(msg)=>println!("{}",msg),
    }
    Account::print_state(&accounts);
}