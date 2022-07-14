
fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first,..,last) => {  //since 'numbers' is a tuple, we take first and last element and ignore others using ..
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}
