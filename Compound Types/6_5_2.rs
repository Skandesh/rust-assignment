
struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u); //Since u is a type of Unit 

    println!("Success!");
} 

//we need to specify the type of arg as Unit
fn do_something_with_unit(u: Unit) {  }
