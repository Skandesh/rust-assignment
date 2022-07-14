
// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names { //passing reference to for loop as .into_iter() as takes ownership of 'names' which can't be accessed after this line, hence only the ref is passed
        println!("{}",name);
    }

    println!("{:?}", names);

    let mut sum = 0; //created a mutable variable for keeping track of array's sum value
    let numbers = [1, 2, 33];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
       sum +=n;
    }
    
    println!("these are the {:?} and sum is {}", numbers,sum);
} 
