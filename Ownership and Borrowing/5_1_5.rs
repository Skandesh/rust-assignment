// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello"); //removed .to_string() as 'String' can't be copied
    let y = x;
    println!("{:?}, {:?}", x, y);
}