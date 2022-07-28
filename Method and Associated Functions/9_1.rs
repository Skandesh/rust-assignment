struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
//Q - Complete the area method which return the area of a Rectangle.
//A - this function should take in instance of 'Rectangle' so it is a method which will accept self as parameter and returns u32 - the area of rectangle
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}
