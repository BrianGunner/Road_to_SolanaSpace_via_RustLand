struct Product{
    name: String,
    price: u32,
    in_stock: bool,
    quantity: u32,
}

impl Product{
    fn new_in_stock(name:String, price:u32)->Product{
        Product { name:name, price:price, in_stock:true, quantity:10 }
    }
    
    fn new_out_of_stock(name:String, price:u32)->Product{
        Product { name:name, price:price, in_stock:false, quantity:0}
    }
}

fn main(){

    let mut products:Vec<Product>= Vec::new();
    products.push(Product::new_in_stock(String::from("Phone"), 50000));
    products.push(Product::new_in_stock(String::from("Laptop"), 50000));
    products.push(Product::new_in_stock(String::from("Mouse"), 1500));
    products.push(Product::new_out_of_stock(String::from("Monitor"), 30000));

  for prod in products.iter(){
    println!("Product: {}, price: {}, In Stock: {}, quantity: {}", prod.name,prod.price,prod.in_stock,prod.quantity);
  }
}