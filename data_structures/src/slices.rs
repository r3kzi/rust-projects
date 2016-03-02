#![allow(dead_code)]
#![allow(unused_variables)]

fn use_slice(slice: &mut[i32]){
    println!("first element = {}, length = {}", slice[0], slice.len());
    slice[0] = 6;
}

pub fn slices() {

    let mut data:[i32;5] = [1,2,3,4,5];
    use_slice(&mut data);
    println!("{:?}", data);
}