
// Fill in the blank
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n == 66 {
           break  //to stop the loop and return the value to 'n'
       }
       n += 1;
    }

    assert_eq!(n, 66);  //comparing if the loop stopped at the right value

    println!("Success!");
}
