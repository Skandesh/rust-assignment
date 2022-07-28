// FILL in the blanks
fn main() {  
    let mut s = String::from("hello, world");
 
    let slice1: &str = &s; // borrowed string
    assert_eq!(slice1, "hello, world");
 
    let slice2 = &s[0..5]; //sliced borrowed string 0 to 4 characters
    assert_eq!(slice2, "hello");
 
    let slice3: &mut String = &mut s; //since value is updated here, it has to be a mutable reference
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
 
    println!("Success!")
 }
 