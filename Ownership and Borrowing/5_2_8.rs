
fn main() {
    // Fix error by modifying this line
    let mut s = String::from("hello, "); //making s mutable since it's passed as mutable reference to the function and if that function mutates any content, original variable's value has to change for that the variable itself has to be mutable at first place!!

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}
