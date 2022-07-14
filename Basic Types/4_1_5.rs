// Fix errors and panics to make it work  -- Fixed the overflow issue by increasing the size
fn main() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }
 