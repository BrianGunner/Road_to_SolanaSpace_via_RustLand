
#[derive(Debug)]
struct Account{
    id:u32,
    balance:i32,
}

fn find_index(accounts:&Vec<Account>,id:u32)->Option<usize>{
    for (index,acc) in accounts.iter().enumerate(){
        if acc.id == id{
            return Some(index);
        }
    }
    None
}

fn main(){
    let mut accounts = vec![
        Account{id:1,balance:0},
        Account{id:2,balance:200},
        Account{id:3,balance:300},
        Account{id:4,balance:400},
        Account{id:5,balance:500},
       ];

    let found_index = find_index(&accounts, 9);

    match found_index {
        Some(value)=>println!("Index Found: {}",value),
        None=>println!("Id not found"),
    }
        
    }

