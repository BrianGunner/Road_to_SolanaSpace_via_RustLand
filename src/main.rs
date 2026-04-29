#[derive(Debug)]
struct Account{
    id:u32,
    balance:i32,
}

fn find_index(accounts:&Vec<Account>,id:u32)->Option<usize>{
    for (index,acc) in accounts.iter().enumerate(){
        if acc.id == id{
            return Some(index)
        }
    }
    None
}

fn transfer(accounts:&mut Vec<Account>,from_id:u32,to_id:u32,amount:i32)->Result<(),String>{
    let from_id_index = find_index(accounts, from_id).ok_or("from_id not found".to_string())?;
    let to_id_index = find_index(accounts, to_id).ok_or("to_id not found".to_string())?;
    if from_id_index==to_id_index{
        return Err("from id cannot be same as to id".to_string());
    }
    if accounts[from_id_index].balance<amount{
        return Err("Insufficient balance".to_string());
    }
    if from_id_index<to_id_index{
       let (left,right) = accounts.split_at_mut(to_id_index);
        left[from_id_index].balance-=amount;
        right[0].balance+=amount;
    }
    else {
        let(left,right)=accounts.split_at_mut(from_id_index);
        right[0].balance-=amount;
        left[to_id_index].balance+=amount;
    }

    Ok(())
}

fn print_state(accounts:&Vec<Account>){
    println!("🤑++++++++++++++++🤑");
    for acc in accounts.iter(){
        
        println!("Account id: {} - Account balance: {}",acc.id,acc.balance);
    
    }
}




fn main(){
    let mut accounts = vec![
        Account{id:1,balance:1000},
        Account{id:2,balance:110},
        Account{id:3,balance:0},
        Account{id:4,balance:111},
        Account{id:5,balance:98898},
    ];
    
    print_state(&accounts);
    let transfer_trial = transfer(&mut accounts, 2, 3, 8);
    match transfer_trial{
        Ok(())=>println!("Success"),
        Err(msg)=>println!("{}",msg),
    }
    print_state(&accounts);

    



}