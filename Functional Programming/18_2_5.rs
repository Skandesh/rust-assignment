fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut(){ // mutably borrows each element of the collection
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}