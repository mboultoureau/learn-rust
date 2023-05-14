#[derive(Debug)] // To use the Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // First parameter of methods is always self of type Self
    // &self is short for "self: &Self"
    // We choose &self because we don't take ownership. &mut self is used to change the instance (here Rectangle).
    // In rare situations, self is used, for example to change the instance into another object and "delete" the instance
    // TLDR: &self: reading, &mut self: mutating, self: consuming
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width >= rectangle.width && self.height >= rectangle.height
    }

    // Not a method because it don't use self as parameter
    // Associated functions are often use as constructor (like here)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let square = Rectangle::square(3);

    println!("Square's area is: {}", square.area());
    println!("Rectangle's area is: {}", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // println!("Rectangle's area is: {} pixels", area(&rect1));

    println!("rect1 is {:?}", rect1); // {:?} Use the Debug trait
    println!("rect1 is {:#?}", rect1); // Same as the previous line but with prettier output (multiline)

    dbg!(&rect1); // same as before but takes the ownership and print the line and filename
    // here we use a reference (&rect1) because we don't want dbg macro takes the ownership of our variable 
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }