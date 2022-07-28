fn main() {
    let mut v = Vec::new();
    for n in 100..200 {
       v.push(n);
    }

    assert_eq!(v.len(), 100);
}