/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
fn main() {
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable);
        //take(&movable);//1st way - passing reference as ownership isn't lost
    };

    consume();
    //consume();  //2nd way - calling it only once
}

fn take<T>(_v: T) {}
