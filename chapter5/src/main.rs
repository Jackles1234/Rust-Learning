#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    }
    
impl Rectangle {         
    fn area(&self) -> u32 { //area function now within the method
    self.width * self.height    //Methods can take ownership of self. WE chose &slef so we don;t take ownership.
    }}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    //println!("rect1 is {}", rect1); Throws an error
    //:? macro is called debug. It enables us to print structs
    //println!("rect1 is {:?}", rect1);
    // for larger structs use {:#?}
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        );
}
fn area(rectangle: &Rectangle) -> u32 {    //only passing one argument
    rectangle.width * rectangle.height
}


/* Default print: 
println!(
    "The area of the rectangle is {} square pixels.",
    area(&rect1)
    );*/