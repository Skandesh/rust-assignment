fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a' with help of iter() and enumerate() 
    for (i,v) in a.iter().enumerate() {  
        println!("The {}th element is {}",i+1,v);
    }
}
