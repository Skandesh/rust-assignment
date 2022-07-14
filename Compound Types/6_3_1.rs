
// Fix the errors, DON'T add new lines!
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; //added reference for s1 slice for rustc to know size at compile time

    let s2: &str = "hello, world";  //removed as_str

    println!("Success!");
}
