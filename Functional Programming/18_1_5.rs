/* Make it work by change the trait bound, in two ways*/
fn fn_once<F>(func: F)
where
    //F: Fn(usize) -> bool,  -- the closure uses the captured value by reference (&T)
    F: FnOnce(usize) -> bool+Copy,  //to implement Copy trait so that value doesn't get moved
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}
