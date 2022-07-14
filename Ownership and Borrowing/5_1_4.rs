// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(&s); //passing the reference of 's' matching the return type

    println!("{}", s);
}

fn print_str(s:&str)  {
    println!("{}",s)
}
