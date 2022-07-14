
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;


    println!("{}",f.data);  //removing the unnecessary placeholders and printing only the accessible element whcih was partially moved
} 
