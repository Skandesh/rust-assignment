
// Fix error with at least two solutions

//1. 
fn main() {
    let s =  "hello, world";
    greetings(s.to_string()) //converted into String
    // greetings(s.to_owned()) -- will work too as we are transferring ownership 
}

fn greetings(s: String) {
    println!("{}",s)
}

//2. 
fn main() {
    let s = String::from("hello, world"); ////converted into String
    greetings(s) 
   
}

fn greetings(s: String) {
    println!("{}",s)
}
