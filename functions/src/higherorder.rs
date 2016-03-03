#![allow(dead_code)]
#![allow(unused_variables)]

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

pub fn higher_order(){
    let limit = 500;
    let mut sum = 0;
    for x in 0.. {
        let isq = x*x;

        if isq > limit { break; }
        else if is_even (isq) { sum += isq; }
    }
    println!("{}", sum);

    let sum2 =
        (0..).map(|x| x*x)
             .take_while(|x| *x<= limit)
             .filter(|x| is_even(*x))
             .fold(0, |sum,x| sum + x);
    println!("sum2 = {}", sum2);
}