fn array_4(arr:[u8;4]){
    println!("Array 4: {:?}",arr)
}
fn array_ref_4(arr:&[u8;4]){
    println!("Array ref 4: {:?}",arr)
}
fn slice_u8(slice:&[u8]){
    println!("Slice of u8: {:?} ",slice)
}
fn main(){
    let arr = [10_u8,20,30,40];
    slice_u8(&arr);
}