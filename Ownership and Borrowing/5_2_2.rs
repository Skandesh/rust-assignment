
fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);  //since y is a reference, to access the value we deferenced it

    println!("Success!");
}
