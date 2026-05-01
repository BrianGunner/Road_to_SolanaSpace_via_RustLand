

const SYSTEM_PROGRAM: u32 = 1;   // Placeholder ID
const OTHER_PROGRAM: u32 = 2;    // Placeholder ID
struct Account{
    address:u32,
    lamports:i32,
    owner:u32,
}

struct Validated{
    from_index:usize,
    to_index:usize,
    amount:i32,
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

fn validate_tx(accounts:&[Account],from_add:u32,to_add:u32,amount:i32)->Result<Validated,String>{
    let from_add_index = find_index(accounts, from_add).ok_or("No from address".to_string())?;
    let to_add_index = find_index(accounts, to_add).ok_or("No from address".to_string())?;
    if from_add_index==to_add_index{
        return Err("From address cannot be same as to address".to_string());
    }
    if accounts[from_add_index].owner!=SYSTEM_PROGRAM{
        return Err("To Owner not System Program".to_string());
    }
    if accounts[to_add_index].owner!=SYSTEM_PROGRAM{
        return Err("To Owner not System Program".to_string());
    }
    if accounts[from_add_index].lamports<amount{
        return Err("Insufficient balance".to_string());
    }
    if amount<=0{
        return Err("Amount cannot be negative".to_string());
    }
    Ok(Validated {from_index:from_add_index, to_index:to_add_index, amount:amount })
}

fn commit_tx(accounts:&mut [Account],test_case:Validated){
    if test_case.from_index<test_case.to_index{
        let (left,right) = accounts.split_at_mut(test_case.to_index);
        {
            left[test_case.from_index].lamports-=test_case.amount;
            right[0].lamports+=test_case.amount;
        }
    }
    else {
        let (left,right) = accounts.split_at_mut(test_case.from_index);
        {
            right[0].lamports-=test_case.amount;
            left[test_case.to_index].lamports+=test_case.amount;
        }
    }
}


fn main()->Result<(),String>{
    let mut accounts = vec![
        Account{address:1,lamports:999,owner:1},
        Account{address:2,lamports:0,owner:1},
        Account{address:3,lamports:23225,owner:2},
        Account{address:4,lamports:34232,owner:1},
        Account{address:5,lamports:0,owner:1},

    ];
    print_state(&accounts);
    let tx_1 = validate_tx(&accounts, 1, 2, 0);
    match tx_1{
        Ok(valid_tx)=>{
            println!("Tx successfully validated");
            commit_tx(&mut accounts, valid_tx);
            println!("Tx commited and changes made");
        },
        Err(msg)=>println!("{}",msg),
    }
    print_state(&accounts);
    Ok(())
}