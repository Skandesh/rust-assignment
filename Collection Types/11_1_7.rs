/*
We know that a String is made up of 3 components:
1. bytes pointer
2. length of the string
3. Capacity

Here we have de-coupled a string into it's individual components and rebuilding it
*/
use std::mem;

fn main() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    // We can re-build a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success! {:?} {:?}", story, s);
}
