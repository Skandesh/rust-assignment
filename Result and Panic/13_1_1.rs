
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!("Program is exited after unwinding all elements/variables in stack");
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}
