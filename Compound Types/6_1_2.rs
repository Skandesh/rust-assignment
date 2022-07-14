
// Fix the error with at least two solutions

//1 - 
fn main() {
    let s: Box<&str> =  "hello, world".into(); //Changing type of Box
    greetings(*s)  //--Deferencing the Box in heap
}

fn greetings(s: &str) {
    println!("{}",s)
}

///--2


fn main() {
    let s: Box<str> =  "hello, world".into();
    greetings(&s) //passing the reference as function expects a &str argument
}

fn greetings(s: &str) {
    println!("{}",s)
}

