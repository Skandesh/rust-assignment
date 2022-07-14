
// Fill the blank
fn main() {
    let mut s = String::from("");  //Allocating a growable String object which is a vector of bytes
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
