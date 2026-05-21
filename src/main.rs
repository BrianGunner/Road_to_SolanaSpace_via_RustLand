fn main(){
    let number:u64 = 12345;
    let data:Vec<u8> = number.to_ne_bytes().to_vec();
    println!("Number stored as data le : {:?}",data);

    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[0..8]);
    let recovered = u64::from_le_bytes(bytes);
    println!("{:?}",recovered);
     
}
