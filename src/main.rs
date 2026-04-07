


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

fn find_index_result(accounts:&Vec<Account>,from_id:u32,to_id:u32)->Result<(usize,usize),String>{
    let from_index = find_index(accounts, from_id).ok_or("From Id not found".to_string())?;
    let to_index = find_index(accounts, to_id).ok_or("To id not found".to_string())?;
    return Ok((from_index,to_index));
   
}

fn main(){
    let mut accounts = vec![
        Account{id:1,balance:0},
        Account{id:2,balance:200},
        Account{id:3,balance:300},
        Account{id:4,balance:400},
        Account{id:5,balance:500},
       ];
    
    let result = find_index_result(&accounts, 2,3);
    match result{
        Ok(value)=>println!("Found index: {:?}",value),
        Err(msg)=>println!("{}",msg),
    }

        
    }

