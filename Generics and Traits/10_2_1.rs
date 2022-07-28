struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    let arrays = [
        Array{
            data: [1, 2, 3, 56],  //whatever number of elements is passed will be the size for all coming instances
        },
        Array {
            data: [1, 2, 3,5],
        },
        Array {
            data: [1, 2,3,5]
        }
    ];

    println!("Success!");
}
