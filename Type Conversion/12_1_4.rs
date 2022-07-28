/*

first_address and second_address are in decimal
p1 and p2 are in hexadecimal 

Changes made at - 
1. line 13 - to adding 1 to the 2nd element of array
2. line 9 - typecasting p1 to usize for storing it's memory address
3. line 16 - converting into hex
*/
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize; 
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; 
    unsafe {
        // add one to the second element
        *p2 = *p2 +1  // p2 is the second element
    }
    
    assert_eq!(values[1], 3);

    println!("Success! p1={:?} p2={:?} first_address={} second_address={}  values = {:?}", p1, p2, first_address, second_address,values);
}
