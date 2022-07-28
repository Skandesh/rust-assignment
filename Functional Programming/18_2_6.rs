
fn main() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();  //mutable borrow

    if let Some(v) = values_iter.next(){  //taking the first iteration, i.e first element of vector
        *v =10;  //deferencing it to update the value on that iteration

    assert_eq!(values, vec![10, 2, 3]);
    println!("Successfully updated vector element with iterators. they are {:?}",values);
}
