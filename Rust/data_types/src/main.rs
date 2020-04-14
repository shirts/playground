#[derive(Debug)]
struct MyTuple ( i32, i8 );

fn main() {
    // tuples
    let tup = MyTuple( 900, 120 );
    println!("tup is {:?}", tup);

    // arrays
    let arr = ["cat", "says", "meow"];
    println!("arr is {:?}", arr);

    let arr2: [i32; 2] = [250, 100];
    println!("arr2 is {:?}", arr2);

    let [ a, b ] = arr2;
    println!("a is {}", a);
    println!("b is {}", b);
}
