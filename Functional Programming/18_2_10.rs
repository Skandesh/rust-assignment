fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x+3).collect();

    assert_eq!(v2, vec![4,5,6]);
}
