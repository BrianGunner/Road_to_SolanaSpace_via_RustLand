fn main(){
    let name = String::from("Solana");
    let name_bytes = name.as_bytes();
    println!("{}",name);
    println!("{:?}",name_bytes);
    let mut data:Vec<u8>=Vec::new();
    for letter in name_bytes{
        println!("Byte: {} Letter: {}", letter, *letter as char);
        data.push(*letter);
    }
    println!("{:?}",data);
    let result = String::from_utf8(data);
    match result{
        Ok(value)=>println!("{}",value),
        Err(msg)=>println!("{}",msg),
    }

}