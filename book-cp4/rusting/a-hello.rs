//! e isso gera algo?

// esconde os warnings de código não usado
#![allow(dead_code)] 
#![allow(unused)]

fn main(){
    print!("Hello World!\n");
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");
    
    println!("Base 10:  {}",   86759);
    println!("Base  2:  {:b}", 86759);
    println!("Base  8:  {:o}", 86759);
    println!("Base 16:  {:x}", 86759);

    println!("a{number:>5}a",  number=1);
    println!("a{number:<5}a",  number=1);
    println!("a{number:0>5}a", number=1);
    println!("a{number:0<5}a", number=1);

    println!("{number:>width$}",  number=1, width=5);

    struct Structures(i32);

    let x: i32 = 4;
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}

// a simple comment

/* a comment with
    more lines */

/// nao entendi, isso gera algo?
fn outra(){
    print!("ah\n");
}

fn coisa(){
    print!("oi\n");
}