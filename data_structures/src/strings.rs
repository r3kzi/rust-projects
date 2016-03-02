#![allow(dead_code)]
#![allow(unused_variables)]

pub fn strings(){
    let s:&'static str = "hello there!";
    for x in s.chars() {
        println!("{}",x);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    //Heap allocated
    //String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("letters = {:?}", letters);

    //&str <> String
    {
        let u:&str = &letters;
        println!("{:?}", u);
    }

    //concatentation
    //String + &str
    let z = letters + "abc";
    println!("{:?}", z);

    let mut abc = "hello_world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));

}