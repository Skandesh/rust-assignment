fn main() {
    let x = Box::new(5); //Box is a smart pointer which allocates memory on heap and size is known only at run-time
    
    let mut y = Box::new(4);      
    
    *y = 42; //Value of y stored in heap is getting dereferenced
    
    assert_eq!(*x, 5);

    println!("Success!");
}