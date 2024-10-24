mod rectangle;

use rectangle::Rectangle;

fn main() {
    let rect1 = Rectangle::new(8, 7);
    let rect2 = Rectangle::new(5, 1);
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
}