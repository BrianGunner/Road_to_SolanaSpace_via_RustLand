
fn divide(a:i64,b:i64)->Result<i64,String>{
    if b==0{
        return Err("Cannot divide by Zero".to_string());
    }
    Ok(a/b)
}


fn main(){
    let output = divide(2, 0);
    match output {
        Ok(value)=>println!("Output is: {}",value),
        Err(msg)=>println!("{}",msg),
        
    }
    let output2 = divide(4,2);
    match output2 {
        Ok(value)=>println!("Output is: {}",value),
        Err(msg)=>println!("{}",msg),
        
    }
    
}