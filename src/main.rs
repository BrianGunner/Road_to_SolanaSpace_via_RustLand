
struct Account{
    id:u32,
    balance:i32,
}
#[derive(Debug)]
struct TransferPlan{
    from_index:usize,
    to_index:usize,
    amount:i32,
}

fn print_state(accounts:&Vec<Account>){
    println!("🤑++++++++++++++🤑");
    for acc in accounts.iter(){
        println!("Id: {} --- Balance: {}",acc.id,acc.balance)
    }
    println!("🤑++++++++++++++🤑");
    
}

fn find_index(accounts:&[Account],id:u32)->Option<usize>{
    for (index,acc) in accounts.iter().enumerate(){
        if acc.id == id{
            return Some(index);
        }
    }
    None
}

fn build_transfer_plan(accounts:&Vec<Account>,from_id:u32,to_id:u32,amount:i32)->Result<TransferPlan,String>{
    let from_index = find_index(accounts, from_id).ok_or("Could not find from id".to_string())?;
    let to_index = find_index(accounts, to_id).ok_or("Could not find to id".to_string())?;
    if from_index == to_index{
        return Err("from id cannot be to id".to_string());
    }
    if accounts[from_index].balance<amount{
        return Err("Insufficient balance".to_string());
    }
    Ok(TransferPlan { from_index,to_index,amount})
}

fn execute_transfer(accounts:&mut Vec<Account>,plan:TransferPlan){
    if plan.from_index<plan.to_index{
        let (left,right )= accounts.split_at_mut(plan.to_index);
        {
            left[plan.from_index].balance-=plan.amount;
            right[0].balance+=plan.amount;
        }
    }
    else if plan.from_index>plan.to_index{
        let (left,right) = accounts.split_at_mut(plan.from_index);
        {
            right[0].balance-=plan.amount;
            left[plan.to_index].balance+=plan.amount;

    }
}
}

fn main()->Result<(),String>{
    let mut accounts = vec![
        Account{id:1,balance:9999},
        Account{id:2,balance:0},
        Account{id:3,balance:78},
        Account{id:4,balance:0},
        Account{id:5,balance:25000},
        Account{id:6,balance:8888},
    ];

    print_state(&accounts);
    let tx_1 = build_transfer_plan(&accounts, 2, 2, 99)?;
    execute_transfer(&mut accounts, tx_1);
    print_state(&accounts);
    return Ok(());

    

   
   

}
