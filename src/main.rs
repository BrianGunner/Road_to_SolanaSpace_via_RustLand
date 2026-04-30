const SYSTEM_PROGRAM: u32 = 1;   // Placeholder ID
const OTHER_PROGRAM: u32 = 2;    // Placeholder ID
struct Account{
    address:u32,
    lamports:i32,
    owner:u32,
}

fn print_state(accounts:&[Account]){
    println!("🤑++++++++++++++++++++++🤑");
    for acc in accounts.iter(){
        println!("Address: {},lamports: {}, owner: {}",acc.address,acc.lamports,acc.owner)
    }
    println!("🤑++++++++++++++++++++++🤑");
}

fn find_index(accounts:&[Account],address:u32)->Option<usize>{
    for (index,acc) in accounts.iter().enumerate(){
        if acc.address==address{
            return Some(index);
        }
    }
    None
}

fn adjust_lamports(accounts:&mut Vec<Account>,address:u32,amount:i32)->Result<(),String>{
    let index_adjust = find_index(accounts, address).ok_or("Id does not exist".to_string())?;
    if accounts[index_adjust].owner==SYSTEM_PROGRAM{
        accounts[index_adjust].lamports+=amount;
        return Ok(());
    }
    Err("Other Program cannot adjust lamports".to_string())
}



fn main(){
    let mut accounts = vec![
        Account{address:1,lamports:999,owner:1},
        Account{address:2,lamports:0,owner:1},
        Account{address:3,lamports:23225,owner:2},
        Account{address:4,lamports:34232,owner:1},
        Account{address:5,lamports:0,owner:1},

    ];
    print_state(&accounts);
    let test_case = adjust_lamports(&mut accounts, 2, 99);
    match test_case{
        Ok(())=>println!("Adjustment done"),
        Err(msg)=>println!("{}",msg),
    }
    print_state(&accounts);
}