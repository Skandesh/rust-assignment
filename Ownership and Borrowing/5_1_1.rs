
fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    //Approach 1
    let y = &x;
   //Approach 2
   let y = x.clone();
   //Approach 3
   let x = "hello world";
   let y = x;
    
    println!("{},{}",x,y);
}
