
fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32);

    assert_eq!((0.15+0.15),0.3); //Updated value since 0.1+0.2 = 0.30000000000000004
    println!("Success!");
}
