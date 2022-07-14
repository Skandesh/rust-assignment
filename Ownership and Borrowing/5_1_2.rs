// Don't modify code in main!
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) ->String {
    println!("{:?}", s);
    s  //Since return type is specified, need to return String object
}
