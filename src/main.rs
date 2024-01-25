mod shapes;
use shapes::{Circle, Rectangle, Shape};

fn main() {
    let mut circle = Circle::new(2.0);
    circle.set_radius(3.0);
    let area = circle.area();
    let perimeter = circle.perimeter();
    println!("area do circulo : {} , perimetro do circulo : {}", area, perimeter);

    let mut rectangle = Rectangle::new(3.0,4.0);
    rectangle.set_width(2.0);
    println!("area do retangulo : {}, perimetro do retangulo : {}", rectangle.area(), rectangle.perimeter());


}
