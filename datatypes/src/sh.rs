#![allow(dead_code)]
use std::mem;

struct Point {
    x:i32, y:i32
}

fn getter() -> Point {
    Point {x: 2, y: 3}
}

pub fn stack_and_heap(){

    let p1 = getter();
    let p2 = Box::new(getter());

    let p3 = *p2;

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p3));
}