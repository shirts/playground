fn main() {
    ownership_1();
    ownership_2();
}


fn ownership_1() {
    let mut bob = String::from("Bob");
    println!("{:?}", bob);

    bob.push_str(", C");
    println!("{:?}", bob);

    let greeting = greet(bob); // bob goes away
    // cannot use bob again
    // greet(bob)
    println!("{}", greeting);
}

fn ownership_2() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = s2; // s2 goes away
    println!("s3 = {}", s3);

    // cannot use s2 again
    // println!("s2 = {}", s2);
    // borrow of moved value: `s2`: value borrowed here after move

    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4); // use of s3 still valid when cloned

}

fn greet(name: String) -> String {
    format!("Hello, {}", name)
}
