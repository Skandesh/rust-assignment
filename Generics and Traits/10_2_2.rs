
// Fill in the blanks to make it work.
fn print_array<T: std::fmt::Debug,const N:usize>(arr:[T;N]) { //specifying the array Type and size of it
    println!("{:?}", arr);
}
fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
