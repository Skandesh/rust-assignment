
fn main() {
    let t = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (s1, s2) = &t;  // letting the reference of tuple to be used by s1 and s2
 
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }
 