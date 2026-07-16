#![allow(unused,dead_code)]
use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    //println!("{}", n);
    let s: String = lines.next().unwrap().unwrap().trim().to_string();
    //println!("{}",s);

    if s.find("lv") != None {
        println!("0");
    } else {
        if s.find("l") != None {
            println!("1");
        } else {
            if s.find("v") != None {
                println!("1");
            } else {
                println!("2");
            }
        }
    }

}