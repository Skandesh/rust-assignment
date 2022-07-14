
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let  mut p = Person { //making the instance as mutable
        name: String::from("sunface"),
        age:age,  //passing the value to the struct's element
    };

    // How can you believe sunface is only 18? 
    p.age = 30;

  
    p.name = String::from("sunfei");  //updated name 

    println!("Success!");
}
