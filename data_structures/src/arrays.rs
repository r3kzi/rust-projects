#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

pub fn arrays(){

    let mut a:[i32;5] = [1,2,3,4,5];

    println!("a has {} elements, first is {}",
        a.len(), a[0]);
    a[0] = 321;
    println!("a[0] = {}", a[0]);
    a[0] = 1;

    println!("{:?}", a);

    if a != [5,6,7,8,9] {
        println!("does not match");
    }

    //minimize the size of the integer
    let b = [1u8;10];
    for x in 0..b.len() {
        println!("{}", b[x]);
    }

    println!("b took up {} bytes",
        mem::size_of_val(&b)
    );

    //matrix
    //xxx
    //xxx
    let mtx:[[f64;3];2] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("{}", mtx[i][j]);
            }
        }
    }

}