struct Account{
    id:u32,
    balance:i32,
}

impl Account{
    fn find_index(accounts:&Vec<Account>,id:u32)->Option<usize>{
        for (index,acc) in accounts.iter().enumerate(){
            if acc.id == id{
                return Some(index);
            }
        }
        None
    }

    fn find_index_ft(accounts:&Vec<Account>,from_id:u32,to_id:u32)->Result<(usize,usize),String>{
        let from_id_index = Account::find_index(accounts, from_id).ok_or("No from_id".to_string())?;
        let to_id_index = Account::find_index(accounts, to_id).ok_or("No to id".to_string())?;
        return Ok((from_id_index,to_id_index));

    }
}





fn main(){
    let mut accounts = vec![
        Account{id:1,balance:9999},
        Account{id:2,balance:0},
        Account{id:3,balance:8878},
        Account{id:4,balance:0},
    ];

    let fun_test = Account::find_index_ft(&accounts, 2, 3);
    let mut left = 0 as usize;
    let mut right = 0 as usize;
    match fun_test{
        Ok(value)=>{
            (left,right) = value;
        },
        Err(msg)=>println!("{}",msg),
    }
    

}