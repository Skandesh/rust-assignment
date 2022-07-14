
// Fill in the blank
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter *20;  // breaking loop and returning the value next to the expression
        }
    };

    assert_eq!(result, 200);

    println!("Success!");
}
