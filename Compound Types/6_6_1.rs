//implemented PartialEq trait to compare equality of custom datatypes.
#[derive(PartialEq)]
enum Number {
    Zero,
    One,
    Two,
}

#[derive(PartialEq)]
enum Number1 {
    Zero = 0,
    One,
    Two,
}

#[derive(PartialEq)]
// C-like enum
enum Number2 {
    Zero = 0.0 as isize,
    One = 1.0 as isize,
    Two = 2.0 as isize,
}


fn main() {
    //changed the enum variant as uint 
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    println!("Success!");
} 
