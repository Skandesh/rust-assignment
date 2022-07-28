/*Changes made at = 
1. line 4 - implemented PartialEq trait as we need to include for custom datatype
2. line 13,14 - added values for enum's elements
 */
#[derive(Debug,PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    // FILL in the blank
    let v : Vec<IpAddr>= vec![
        IpAddr::V4(String::from("127.0.0.1")),
        IpAddr::V6(String::from("::1"))];
    
    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}
