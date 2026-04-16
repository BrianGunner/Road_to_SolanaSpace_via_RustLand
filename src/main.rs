#[derive(Debug)]
struct Account{
    id:u32,
    balance:i32,
}

fn find_account_index(accounts:&mut Vec<Account>,from_id:u32,to_id:u32)->Result<(usize,usize),String>{
    let mut from_id_index = None;
    let mut to_id_index = None;

    for (index,acc) in accounts.iter().enumerate(){
        if from_id == acc.id{
            from_id_index = Some(index);
        }
        
    }

    for (index,acc) in accounts.iter().enumerate(){
        if to_id == acc.id{
            to_id_index = Some(index);
        }
    }
    let mut from_id_index_final = from_id_index.ok_or("Could not find from_id".to_string())?;
    
    let mut to_id_index_final = to_id_index.ok_or("Could not find to_id".to_string())?;
    
    return Ok((from_id_index_final,to_id_index_final));

}

fn main(){
    let mut accounts = vec![
        Account{id:1,balance:0},
        Account{id:2,balance:99},
        Account{id:3,balance:25000},
    ];

    let found = find_account_index(&mut accounts, 3, 1);
    let mut from_index= 0 as usize;
    let mut to_index = 0 as usize;
    match found {
        Ok(value)=>{(from_index,to_index) = value;},
        Err(msg)=>println!("{}",msg),
        
    }
    let amount = 500;
    let (left,right) = accounts.split_at_mut(2);
    println!("left = {:?},right = {:?}",left,right);
    {
        right[0].balance +=amount;
        left[0].balance += amount;
    }

    

}