#![allow(unused,dead_code)]

use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    //let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    while let Some(Ok(linha)) = lines.next() {
        let valores: Vec<i32> = linha
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        print!("{:?}\n", (2*valores[0]*valores[1]));
    }

}