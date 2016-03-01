#![allow(dead_code)]
#![allow(unused_variables)]

pub fn options(){
    // Option<T>

    let x = 3.0;
    let y = 2.0;

    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else { None} ;

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide")
    }

    // if let / while let
    if let Some(z) = result { println!("z = {}", z);}
    while let Some(z) = result { println!("z = {}", z); break;}
}