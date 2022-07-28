// To use `from_str` method, you needs to introduce this trait into the current scope.
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed:i32 = "10".parse().unwrap();
    let from_str = "20".parse::<i32>().unwrap();
    let another_str = i32::from_str("10").unwrap();
    let sum = parsed + turbo_parsed + from_str + another_str;
    
    assert_eq!(sum, 45);

    println!("Success!")
}
