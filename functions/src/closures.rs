#![allow(dead_code)]
#![allow(unused_variables)]

fn say_hello(){
    println!("hello");
}

pub fn closures(){
    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 { x+1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 1;

    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", a, plus_two(a));
    }

    let borrow_two = &mut two;

    let plus_three = |x:&mut i32| { *x += 1 };
    let mut c = 9;
    plus_three(&mut c);
    println!("{}", c);
}