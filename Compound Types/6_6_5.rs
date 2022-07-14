
// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn main() {
    let five = Some(5);  //five has a type of Option<i32> 
    let six = plus_one(five); //plus_one()  has been called passing the Option<i32> in it 
    let none = plus_one(None); //plus_one()  has been called passing the None  in it which has type of Option<i32>

    if let Some(n) = six {  //since 'six' is of type 
        println!("{}", n);

        println!("Success!");
    } 
        
    panic!("NEVER LET THIS RUN！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
         None => None,
        Some(i) => Some(i + 1),
    }
}
