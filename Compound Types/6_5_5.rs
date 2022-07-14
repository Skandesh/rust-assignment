
// Fill the blank
struct Person {
    name: String,
    age: u8,
}
fn main() {
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name  //passed the argument into struct's instance, order doesn't matter and this is init shorthand syntax to avoid repititions
    }
}
