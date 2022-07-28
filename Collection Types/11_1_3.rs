
// Question: how many heap allocations are happened here ?
// Your answer: 
fn main() {  
    // Create a String type based on `&str`
    // the type of string literals is `&str`
   let s: String = String::from("hello, world!"); //1st allocation in heap

   // create a slice point to String `s`
   let slice: &str = &s;

   // create a String type based on the recently created slice
   let s: String = slice.to_string(); //2nd allocation, shadow variable 

   assert_eq!(s, "hello, world!");

   println!("Success!")
}
