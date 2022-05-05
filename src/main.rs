use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let s1 = String::from("tic");
    let mut hash_map2 = HashMap::new();
    hash_map2.insert(1, 2);
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    let s8 = s1 + &s2 + &s3;
}
