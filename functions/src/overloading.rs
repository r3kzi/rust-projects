#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Add for Point {

    type Output = Point;

    fn add(&self, other:Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

pub fn overloading(){
    let p1 = Point { x:2.0, y: 3.0};
    let p2 = Point { x:4.0, y: 6.0};
    let p3 = p1 + p2;
    println!("{:?}", p3);
}