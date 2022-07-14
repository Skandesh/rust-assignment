
fn main() {
    for n in 1..100 {  //removed '=' to make 100 exclusive and it won't enter if block to crash the program(panic)
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 
