/*
Changes made at - 
1. line  9 - mentioned the type of variable 
2. line 12 - as only u8 can be converted to char
3. line 17 - adjusted the value such that assert succeeds
*/

fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = integer as char;
   
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8  -1);
    
   

    println!("Success! {} {}", decimal, integer)
}
