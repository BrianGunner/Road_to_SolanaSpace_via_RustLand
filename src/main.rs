

fn main(){
   let mut data:Vec<u8> = Vec::new();
   let num1= 1001_u64;
   let num1_as_bytes = num1.to_le_bytes();
   for byte in num1_as_bytes{
        data.push(byte);
   }
   println!("{:?}",data);
   let num2: u64 = 5000;
   let num2_as_bytes = num2.to_le_bytes();
   for bytes in num2_as_bytes{
    data.push(bytes);
   }
   println!("{:?}",data);

   let mut bytes_1 = [0u8;8];
   bytes_1.copy_from_slice(&data[0..8]);
   let mut bytes_2 = [0u8;8];
   bytes_2.copy_from_slice(&data[8..16]);
   let recover_1 = u64::from_le_bytes(bytes_1);
   let recover_2 = u64::from_le_bytes(bytes_2);
   println!("Number 1 after conversion and reconversion: {}",recover_1);
   println!("Number 2 after conversion and reconversion: {}",recover_2);
}






