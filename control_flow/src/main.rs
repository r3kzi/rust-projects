fn if_statement() {
    let temp = 25;

     if temp > 30 {
         println!("really hot outside!");
     } else if temp < 10 {
         println!("really cold outsite!");
     }
    else {
         println!("temperature is ok!");
     }

    let day = if temp >= 20 {"sunny"} else {"cloudy"};
    println!("{}", day);

    println!("it is {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"}
    );

    println!("it is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else {"cold"}
    );
}

fn while_loop() {
    let mut x = 1;

    while x < 100 {
        x *= 2;

        if x == 64 {
            continue;
        }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop {
        //while true
        y*=2;
        println!("y = {}", y);

        if y == 1<<10 { break; }
    }
}

fn for_loop() {
    for x in 1..11 {

        if x == 3 { continue; }

        println!("x = {}", x);
    }

    for (pos,y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement(){
    let country_code = 999;

    let country = match country_code {
        44 => "UK",
        49 => "GER",
        1...999 => "UNKNOWN",
        _ => "INVALID"
    };
    println!("the country with code {} is {}", country_code, country);
}


fn main() {
    match_statement();
    for_loop();
    while_loop();
    if_statement();
}