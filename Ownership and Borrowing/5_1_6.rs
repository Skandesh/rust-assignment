
fn main() {
    let s = String::from("hello, ");
    
    // Modify this line only !
    let mut s1 = s; //Default variables will be immutable

    s1.push_str("world");

    println!("Success!");
}
