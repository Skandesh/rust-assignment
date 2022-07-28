
// modify the code below to print out: 
// 25
// 25
// 25
// Here, there’s no need to allocate more memory inside the loop.

/*
Usually strings are stored in heap memory and they usually are presented in form of blocks which is designed by the system'sarchitecture itself.
Memory blocks are generally grouped as 8 (or) 16 bits. 
Capacity is space of one such block where future elements will be added into it

Here we have specified the capacity to be 25, even if there are no string/elements it's length would be 0 but the capacity remains 25 as specified.
if the vector's length crosses 25, the capacity would be automatically reallocated to some other memory location.
*/
fn main() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
       
        s.push_str("hello");
         println!("{}", s.capacity());
        
    }

    println!("Success!")
}
