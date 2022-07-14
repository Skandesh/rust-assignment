
// Fill in the blank and fix the error
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move{x: 42, y: 42};  //Updated the value for x & y

    if let Message::Move{x:a, y:b} = msg { //filled the blanks for the value
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
} 
