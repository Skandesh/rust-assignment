fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);  //s is not moved and still can't be accessed after this line, as only reference is passed
    
    s.push_str("world"); //like this for instance

    println!("Success!");
}

fn borrow_object(s: &String) {}