
// Fix all errors without adding newline
fn main() {
    let  mut s =  String::from("hello"); //made it mutable
    s.push(',');
    s.push_str(" world");  //changed to push_str as it accepts String and pushes
    s += "!";  //removed to_string() as only char is added

    println!("{}", s);
}
