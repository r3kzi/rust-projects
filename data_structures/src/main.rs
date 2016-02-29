#![allow(dead_code)]
#![allow(unused_variables)]

mod enums;

struct Point {
    x:f64,
    y:f64
}

fn structures() {
    let p1 = Point {x: 3.0, y: 4.0};
    println!("point p1 is at {} and {}", p1.x, p1.y);

    let p2 = Point {x: 5.0, y: 10.0};
    let my_line = Line {start: p1, end: p2};
}

struct Line {
    start: Point,
    end: Point
}

fn main() {
    enums::enums();
    println!("---------------------------------------");
    structures();
}