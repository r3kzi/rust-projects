#![allow(dead_code)]
#![allow(unused_variables)]


struct Point<T> {
    x: T,
    y: T
}

pub fn generics() {
    let a = Point {x: 0, y: 0};
    let b = Point {x: 1.2, y: 3.4};
}