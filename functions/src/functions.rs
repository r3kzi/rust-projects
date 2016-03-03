#![allow(dead_code)]
#![allow(unused_variables)]

pub fn functions() {
    print_value(12);
    let mut z = 1;
    increace(&mut z);
    println!("final value of z = {}", z);

    let a = 3;
    let b = 3;
    let p = product(a,b);
    println!("{}", p);
}

fn print_value(x: i32){
    println!("value = {}", x);
}

fn increace(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}