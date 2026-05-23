fn main(){
    let name = "Solana";
    let name_bytes = name.as_bytes();
    println!("Solana as &[u8] : {:?}",name_bytes);
    let mut data:Vec<u8> = Vec::new();
    for letters in name_bytes{
        data.push(*letters);
    }
    println!("Solana in Vec<u8>: {:?}",data);

    let recoverd_string = String::from_utf8(data).unwrap();
    println!("recoverd_string: {}",recoverd_string);

}