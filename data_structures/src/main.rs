#![allow(dead_code)]
#![allow(unused_variables)]

mod enums;
mod options;
mod arrays;
mod vectors;
mod slices;
mod strings;
mod tuples;
mod pm;
mod generics;

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
    vectors::vectors();
    println!("---------------------------------------");
    arrays::arrays();
    println!("---------------------------------------");
    options::options();
    println!("---------------------------------------");
    enums::enums();
    println!("---------------------------------------");
    structures();
    println!("---------------------------------------");
    slices::slices();
    println!("---------------------------------------");
    strings::strings();
    println!("---------------------------------------");
    tuples::tuples();
    println!("---------------------------------------");
    pm::pattern_matching();
    println!("---------------------------------------");
    generics::generics();

}