#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    println!(
        "The area of the rectangle is {} aquare pixels.",
        rect1.area()
    );
    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle { width: 30, height: 50 };
    let rect3 = Rectangle { width: 10, height: 40 };
    let rect4 = Rectangle { width: 60, height: 45 };

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect4));
}

