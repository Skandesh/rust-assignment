/*
Changes at -
1.  line 10 - to add the right end limit
2. line 20 - extend range
3. line 21 - commented the line as slices are read only
 */
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // out of bounds will cause a panic
    // You must use `v.len` here
    let slice2 = &v[0..v.len()];
    
    assert_eq!(slice1, slice2);
    
    // slice are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..4];
  //  slice3.push(4);

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!")
}
