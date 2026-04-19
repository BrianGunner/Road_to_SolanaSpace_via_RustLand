#[derive(Debug)]
struct Account{
    id:u32,
    balance:i32,
}

fn index_find(accounts:& Vec<Account>,id:u32)->Result<usize,String>{
    for (index,acc) in accounts.iter().enumerate(){
        if acc.id == id{
            return Ok(Some(index).ok_or("Id not found,None,ok_or".to_string())?);
        }
    }
    Err("Id not found,Err".to_string())
}

fn main(){
    let accounts = vec![
        Account{id:1,balance:25000},
        Account{id:2,balance:0},
        Account{id:3,balance:3434},
        ];

    let found = index_find(&accounts, 8);

    match found {
        Ok(value)=>println!("{}",value),
        Err(msg)=>println!("{}",msg),
        
    }

     
}