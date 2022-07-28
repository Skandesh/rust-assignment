struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
   
    fn mixup<A,B>(self, alt:Point<A,B>)->Point<T,B>{ //alt - other instance of Point and that has its own generic types A,B
        Point{
            x:self.x,
            y:alt.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}
