/* Make it work with least changing */
fn main() {
    let color = String::from("green");

    let print =  || println!("`color`: {}", color); //removing move and declaring a empty closure

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    let _reborrow = &color;

    println!("{}",color);
}
