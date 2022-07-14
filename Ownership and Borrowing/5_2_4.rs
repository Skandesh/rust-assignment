
// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s); //passing mutable reference 

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
