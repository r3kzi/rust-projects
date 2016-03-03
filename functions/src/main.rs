#![allow(dead_code)]
#![allow(unused_variables)]

mod functions;
mod closures;
mod higherorder;
mod traits;
mod overloading;


fn main(){
    functions::functions();
    println!("----------------------------------------");
    closures::closures();
    println!("----------------------------------------");
    higherorder::higher_order();
    println!("----------------------------------------");
    traits::traits();
    println!("----------------------------------------");
    overloading::overloading();

}