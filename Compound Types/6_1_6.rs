
// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;  //Since value would be moved, clone of s1 is taken and also to concatenate passed the ref of s2
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}
