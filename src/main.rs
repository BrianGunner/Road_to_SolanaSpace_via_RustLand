struct Account{
    id:u32,
    balance:i32,
}

fn first_1000(accounts:&Vec<Account>)->Result<i32,String>{
    for acc in accounts.iter(){
        if acc.balance>1000{
            return Ok(acc.balance);
        }
    }
    Err("Could not find".to_string())
}

fn add_100(accounts:&Vec<Account>)->Result<i32,String>{
    let result = first_1000(accounts)?+100;
    Ok(result)
}

fn main()->Result<(),String>{
    let accounts = vec![
        Account{id:1,balance:999},
        Account{id:2,balance:0},
        Account{id:3,balance:887},
        Account{id:4,balance:111},
    ];
    
let result_fetch = add_100(&accounts)?;
println!("{}",result_fetch);
Ok(())
    
    

}