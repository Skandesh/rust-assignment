
fn main() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);  //Only 't' and 't.0' is moved out, but still we can access 't.1'
 }
 