
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x<split => assert!(x < split), // adding a condition which matches assert!()
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}
