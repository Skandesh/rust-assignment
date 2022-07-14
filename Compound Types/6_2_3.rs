
fn main() {
    // Fill the blank
    let list: [i32; 100] = [1;100] ; //100 elements with value 1 is created

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
