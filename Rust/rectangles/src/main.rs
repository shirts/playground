struct Rectangle {
    width: u64,
    height: u64
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 30,
        height: 50
    };


    println!("The area of rect1 is: {}", rect1.area());
    println!("The area of rect2 is: {}", rect2.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
}

