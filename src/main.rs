 #[derive(Debug)]
struct TokenAccountData{
    balance:u64,
    frozen:bool,
}

fn main(){
    let mut data: Vec<u8> = Vec::new();
    let tokenaccountdata_1 = TokenAccountData{balance:1000,frozen:true};
    let balance_bytes = tokenaccountdata_1.balance.to_le_bytes();
    for bytes in balance_bytes.iter(){
        data.push(*bytes);
    }
    if tokenaccountdata_1.frozen==true{
        data.push(1);
    }
    else{
        data.push(0);
    }
    println!("{:?}",data);

    let mut rebalance_bytes = [0u8;8];
    
    rebalance_bytes.copy_from_slice(&data[0..8]);

    let rebalance = u64::from_le_bytes(rebalance_bytes);
    
    let re_frozen_bytes = data[8];

    let mut re_frozen = false;

    if re_frozen_bytes == 1{
        re_frozen=true;
    }

    let re_tokenaccountdata_1 = TokenAccountData{balance:rebalance,frozen:re_frozen};
    println!("{:?}",tokenaccountdata_1);
    println!("{:?}",re_tokenaccountdata_1);

}