#![allow(dead_code)]
#![allow(unused_variables)]

pub fn vectors() {

    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(4);

    println!("a = {:?}", a);

    let idx:usize = 0;

    a[idx] = 123;

    println!("a[0] = {}", a[idx]);

    match a.get(6) {
        Some(x) => println!("a[6] = {},", x),
        None => println!("none suche element")
    }

    for x in &a {
        println!("{}", x);
    }

    let last_element = a.pop(); //Option
    println!("last element = {:?}, a = {:?}", last_element, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}