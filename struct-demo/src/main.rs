#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn circumference(&self) -> f64 {
        (self.width + self.height) * 2f64
    }

    fn make(width: f64, height: f64) -> Rectangle {
        Rectangle{
            width,
            height
        }
    }

    fn square(size: f64) -> Rectangle{
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::make(1.5, 2.5);
    let area = rect1.area();
    let circumference = rect1.circumference();

    println!("Rectangle: {:#?}", rect1);
    println!("Area: {}", area);
    println!("Circumference: {}", circumference);
}

