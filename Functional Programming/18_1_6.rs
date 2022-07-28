fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

//specifying the lifetime and the argments for FnMut
fn exec<'a, F: FnMut( &'a str)>(mut f: F)  {
    f("hello")
}
