#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8)
}

pub fn enums(){
    println!("enums");
    println!("---------------------------------------");

    let c:Color = Color::RgbColor(10,0,0);

    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RgbColor(0,0,0) => println!("black"),
        Color::RgbColor(r,g,b) => println!("rbg({},{},{})", r, g, b)
    }
}
