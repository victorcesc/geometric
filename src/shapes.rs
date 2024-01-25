use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Circle {
    radius: f64
}

impl Circle {
    pub fn new(radius_value : f64) -> Circle {
        Circle {radius : radius_value}
    }
    pub fn get_radius(&self) -> f64{
        return self.radius;
    }
    pub fn set_radius(&mut self, new_radius: f64){
        self.radius = new_radius;
    }
}

pub struct Rectangle {
    width : f64,
    height : f64
}

impl Rectangle {
    pub fn new(width_value : f64, height_value : f64) -> Rectangle {
        Rectangle {width: width_value, height: height_value}
    }
    pub fn get_width(&self) -> f64 {
        return self.width;
    }
    pub fn get_height(&self) -> f64 {
        return self.height;
    }
    pub fn set_height(&mut self, new_height: f64) {
        self.height = new_height;
    }
    pub fn set_width(&mut self, new_width: f64) {
        self.width = new_width;
    }
}


impl Shape for Circle {

    fn area(&self) -> f64 {
        return PI * self.radius.powi(2);
    }
    fn perimeter(&self) -> f64 {
        return 2.0 * PI * self.radius;
    }

}

impl Shape for Rectangle {
    
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
    fn perimeter(&self) -> f64 {
        return self.width * 2.0 + self.height * 2.0;
    }
}