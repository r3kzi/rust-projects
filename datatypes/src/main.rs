use std::mem;

//global variables
const MEANING_OF_LIFE:u8 = 42;
static z:i32 = 2;

fn scope_and_shadowing(){
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);
        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
}

fn operators() {
    //arithmetic
    let mut a = 2+3*4;
    println!("{}", a);

    a = a+1;
    println!("{}", a);
    a -= 2;
    println!("{}", a);
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let b = 2.5;
    let b_cubed = f64::powi(b,3);
    let b_to_pi = f64::powf(b,std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //bitwise operators
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                   // 01 OR 10 == 11 == 3_10
    println!("1|2 = {}", c);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
    println!("{}", pi_less_4);
}

fn datatypes() {
    //unsigned 0 or positive
    let number_unsigned:u8 = 123; //8bits
    //signed integer
    let number_signed:i8 = -123;

    println!("{}",number_unsigned);
    println!("{}",number_signed);

    //mutable variable
    let mut mutable_number:u8 = 12;
    println!("{}",mutable_number);
    mutable_number = 34;
    println!("{}",mutable_number);

    //type inference
    let type_inference = 123456789;
    println!("{} with bytes = {}",type_inference, mem::size_of_val(&type_inference));

    //char
    let letter:char = 'x';
    println!("{}",letter);

    //boolean
    let boolean = false;
    println!("{}",boolean);
}

fn main() {
    scope_and_shadowing();
    println!("-----------------------------------------");
    operators();
    println!("-----------------------------------------");
    datatypes();
}