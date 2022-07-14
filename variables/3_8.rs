// Fix the error below with least amount of modification
fn main() {
    let (mut x, y) = (1, 2);  //Either making it mutable
    x += 2;

    let x = 5;   //Or creating shadow for x

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
