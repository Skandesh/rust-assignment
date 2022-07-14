
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) { //matching vector values using matches!
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}
