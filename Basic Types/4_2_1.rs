
// Make it work
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);  //Each character would occupy 4bits/1byte

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");
} 
