
 fn create_fn() -> impl FnOnce(i32) ->i32 {
    let num = 5;

    move |x| x + num
}


fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
