
// Fill in the blanks
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           continue;  ///continues if the desired condition is not met, in this case value shouldn't be 66
       }
       
       break  // this will run when 'n' value becomes 66
    }

    assert_eq!(n, 66);

    println!("Success!");
}
