/* Refactoring the following code using iterators */
fn main() {
    let arr = [0; 10];
    for i in 0..arr.into_iter().len() {
        println!("{}",arr[i])
    }
}
