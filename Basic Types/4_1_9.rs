fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
       
    }
    assert!(sum == -5);  // as -3..2 runs 5 times and value of sum would be -3 -5 -6 -6 -5 after 5 iterations
    for c in 'a'..='z' {
      println!("{}",c as u8); //converts character into correspoding ASCII value
   }
}
