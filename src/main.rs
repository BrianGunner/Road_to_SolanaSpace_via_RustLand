fn main(){
    let name = "Solana";
    let name_bytes = name.as_bytes();
    for byte in name_bytes{
        println!("{} - {}",byte, *byte as char)
    }
    let name_2 = "Solana";
    let u16_bytes: Vec<u16> = name_2.encode_utf16().collect();  // Each char as u16
    println!("{:?}",u16_bytes)

}