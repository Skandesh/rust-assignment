fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {           //else-if  block is included
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
} 
