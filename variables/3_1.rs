// Fix the error below with least amount of modification to the code
fn main() {
  //  let x: i32;  Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

//My solution

    let x = 5; //Here value 5 is binded to x and stored in stack, if it's uninitlialized no value would be present in that location and accessing that memory would cause an error!
    assert_eq!(x, 5);
    println!("Success!");
}
