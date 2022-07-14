
fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let  mut p = s;  //making p mutable and moving s to it (s is no longer valid after this line)
    
//Solution 2 - let mut p = &mut s  

    p.push_str("world");

    println!("Success!");
}
