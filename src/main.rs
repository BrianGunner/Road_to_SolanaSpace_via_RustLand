struct Account{
    id:u32,
    balance:i32,
}

fn find_1000(accounts:&Vec<Account>)->Result<i32,String>{
    for acc in accounts.iter(){
        if acc.balance>=1000{
            return Ok(acc.balance);
        }
    }
    Err("Could not find acc above 1000".to_string())
}

fn find_halve(accounts:&Vec<Account>)->Result<i32,String>{
    let result_halve = find_1000(accounts)?;
    Ok(result_halve/2)
}

fn find_halve_even(accounts:&Vec<Account>)->Result<i32,String>{
    let result_halve_even = find_halve(accounts)?;
    if result_halve_even%2==0{
        Ok(result_halve_even)
    }
    else{
        Err("Result not even".to_string())
    }
}

fn main()->Result<(),String>{
    let accounts = vec![
        Account{id:1,balance:1000},
        Account{id:2,balance:0},
        Account{id:3,balance:887},
        Account{id:4,balance:111},
    ];

    let final_result = find_halve_even(&accounts)?;
    println!("{:?}",final_result);
    Ok(())
    
 

}