

fn divide(a:i32,b:i32)->Result<i32,String>{
    if b==0{
        Err("Cannot divide by zero".to_string())
    }
    else{
        Ok(a/b)
    }
}

fn double_divide(a:i32,b:i32){

    let result = divide(a, b);
    match result{
        Ok(value)=>println!("{}",value*2),
        Err(msg)=>println!("{}",msg),
    }

}
    

fn main(){
    let result = divide(60, 20);
    match result{
        Ok(value)=>println!("result : {}",value),
        Err(msg)=>println!("{}",msg),
    }

    double_divide(60,20);

}