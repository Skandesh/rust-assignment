
// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Dont't add/remove any code line
fn main() {
    let mut s: String = String::from("hello, ");  //Converting &str to String
    s.push_str("world");
    s.push('!');  //since only a character is added, changed it to single quotes

    move_ownership(s.clone());  //since value would be moved, we are cloning and passing the value - takes extra space but does the job for this excersice 

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
