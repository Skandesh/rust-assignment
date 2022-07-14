
// Comment one line to make it work
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
  //  println!("{}",r1);  //this would work
  // println!("{}",r2); -- prints - hello, world! - last mutable reference
//    println!("{}",r1); --throws an error as below this another mutable refernce is happening for s in r2
}
