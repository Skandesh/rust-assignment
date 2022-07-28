fn get_person() -> String {
    String::from("skandesh")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}


fn main() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("skandesh", 69.12), ("div", 90.34)];
    /* Make it print:
    skandesh:   69.1
    div:   90.3
    */
    for (name, score) in scores {
        println!("{name}: {score:width$.precision$}");
    }
}