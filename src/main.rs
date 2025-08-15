// Used Debug trait 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// created method implementation for the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    // created instance for the rectangle
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    println!("The area of the rectangle {}", rect1.area());
}
