
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
       value =>{ value.push_str(" world!") ; //removed &mut as during matching the value gets moved and 'r' is only a &mut
       println!("{}",value);
       }
    }
}
