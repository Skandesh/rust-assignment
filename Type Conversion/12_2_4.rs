// TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::TryInto;

fn main() {
    let n: i16 = 2646;  // i16's maximum limit is 32767 - so it can have any have between 0-32767


  //Since we are converting the type into 'u8' it's maximum range is 255 and hence overflow
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success! {}",n);
}
