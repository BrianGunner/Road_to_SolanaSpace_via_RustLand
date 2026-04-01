

fn add_two_numbers(a:i32,b:i32)->Result<i32,String>{
    if a > 0 && b > 0{
        Ok(a+b)
    }
    else{
        Err("Cannot add negative number".to_string())
    }

}

fn add_and_double(a:i32,b:i32)->Result<i32,String>{
    let result = add_two_numbers(a, b)?;
    Ok(result*2)
}

fn add_and_double_and_add(a:i32,b:i32)->Result<i32,String>{
    let result_add = add_two_numbers(a, b)?;
    let result_double = add_and_double(a, b)?;
    Ok(result_add+result_double)
}



fn main()->Result<(),String>{
   let value =  add_two_numbers(10,20)?;
   println!("Sum of two numbers is: {}",value);
   Ok(())
   


}