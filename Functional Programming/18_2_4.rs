
fn main() {
    let arr = vec![0; 10];
    for i in arr.iter() {  //iter borrows each element of the collection through each iteration  and available for reuse after the loop
        println!("{}", i)
    }

    println!("{:?}",arr);
}
