
fn main() {
    let boolean = true;

    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean{
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}
