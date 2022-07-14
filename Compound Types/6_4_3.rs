
// Fix the error
fn main() {
    let too_long_tuple = ( 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);//only 12 elements can be printed
    println!("too long tuple: {:?}", too_long_tuple);
}
