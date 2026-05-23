fn main(){
    let number: u64 = 123456;
    println!("Number {}",number);
    let data:Vec<u8> = number.to_le_bytes().to_vec();
    println!("Store Data is : {:?}",data);

    let mut bytes = [0u8;8];
    bytes.copy_from_slice(&data[0..8]);
    let recovered = u64::from_le_bytes(bytes);
    println!("{:?}",recovered);
}