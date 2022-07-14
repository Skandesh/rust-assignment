
fn main() {
    let x = 5;
    // Fill the blank
    let p = &x  //borrowed value of x and p points to address where x is stored;
 
    println!("the memory address of x is {:p}", p); // Address pointer/value where x actually lives
 }
 