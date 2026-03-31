

fn add_two_numbers(a:i32,b:i32)->Result<i32,String>{
    if a > 0 && b > 0{
        Ok(a+b)
    }
    else{
        Err("Cannot add negative number".to_string())
    }

}

fn add_and_double(a:i32,b:i32)->Result<i32,String>{
    match add_two_numbers(a, b){
        Ok(value)=>Ok(value*2),
        Err(msg)=>return Err(msg),
    }
}

fn add_and_double_and_add(a:i32,b:i32)->i32{
    //take a+b, then take double and then add both again
    let mut add = 0;
    let mut double = 0;
    match add_two_numbers(a, b){
        Ok(value)=>add=value,
        Err(msg)=>(),
    }
    match add_and_double(a, b){
        Ok(value)=>double=value,
        Err(msg)=>()
    }
    return add+double;

}



fn main(){

    let a=-10;
    let b = 20;

    let result = add_two_numbers(a,b);
    match result{
        Ok(value)=>println!("The result is : {}",value),
        Err(msg)=>println!("{}",msg)
    }

    let result_2 = add_and_double(a, b);
    match result_2 {
        Ok(value)=>println!("Doubled value: {}",value),
        Err(msg)=>println!("{}",msg),
    }

    let add_plus_double = add_and_double_and_add(a, b);
    println!("Add and double and add is : {}",add_plus_double);

}