
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &r1;//since two variables can't change the content at once rustc allows only one mutable reference 

    println!("{}, {}", r1, r2);

    println!("Success!");
}


//THis is will work too!!!!

fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;  ///there can be infinite number of immutable ref

    println!("{}, {}", r1, r2);

    println!("Success!");
}