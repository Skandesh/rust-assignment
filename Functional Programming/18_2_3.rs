fn main() {
    let v1 = vec![1, 2];
    let mut v2 = v1.into_iter(); //value gets moved here as it consumes the collection


    assert_eq!(v2.next().unwrap(), 1);
    assert_eq!(v2.next().unwrap(), 2);
    assert_eq!(v2.next(), None);
    println!("Successfully iterated over the vector");
}
