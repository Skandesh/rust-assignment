struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.

    //Passing a borrowed instance of self
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.

    //Passing a mutable borrowed instance of self as value of color is getting modified here
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}
