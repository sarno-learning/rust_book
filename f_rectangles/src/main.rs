#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn new_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let rectangle = Rectangle::new(30, 50);
    let larger_rectangle = Rectangle::new_square(70);

    println!(
        "The area of the rectangle is {} sq pixels.\n",
        rectangle.area()
    );
    println!(
        "Can the first hold the second? {}",
        rectangle.can_hold(&larger_rectangle)
    );
    println!(
        "Can the second hold the first? {}\n",
        larger_rectangle.can_hold(&rectangle)
    );

    let skinny_rectangle = Rectangle::new(20, 90);
    println!("Skinny Rectangle: {:#?}", skinny_rectangle);

    let tall_rectangle = Rectangle::new(70, 10);
    println!("Tall Rectangle: {:#?}", tall_rectangle);

    let max_rectangle = skinny_rectangle.max(tall_rectangle);
    println!("Max Rectangle: {:#?}\n", max_rectangle);

    let mut mutable_rectangle = Rectangle::new_square(10);
    println!("Before Update: {:#?}", mutable_rectangle);

    mutable_rectangle.set_height(20);
    mutable_rectangle.set_width(30);
    println!("After Update: {:#?}", mutable_rectangle);
}
