/*
Changes at -
1. line 7 - defined the return type of multiply() as Result
2. line 16 - since it returns a Result, value has to be enclosed with Ok()
3. line 19 and 20 - updating parameters and unwrapping to be compared with an value
*/
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!")
}
