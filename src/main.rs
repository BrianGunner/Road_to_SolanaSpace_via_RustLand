fn main(){
    let fixed = 100_u8;
    let fixed_le_bytes = fixed.to_le_bytes();
    println!("Fixed as bytes: {:?}",fixed_le_bytes);
    let mut growable = fixed_le_bytes.to_vec();
    growable.push(99);
    growable.push(88);
    growable.pop();
    println!("Fixed to growable {:?}",growable);
    let name = "String";
    let name_as_u8 = name.as_bytes();
    println!("{:?}",name_as_u8);
}