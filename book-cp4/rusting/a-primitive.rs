fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer: i32 = 5i32;

    let def_float = 3.0;
    let def_integer = 7;

    let mut inferred_type = 12; //i32
    inferred_type = 4294967296i64; //i64

    let mut mutable = 12;
    mutable = 21;

    //mutable = true; // ERROR
    
    let mutable = true; // ok; shadowing

    //compound types

    let my_array: [i32; 5] = [1,2,3,4,5]; //mesmo tipo

    let my_typle = (5u32, 1u8, true, -5.04f32); //tipos diferentes

    //literais e operadores

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    println!("true and false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("not true is {}", !true);

    println!("0011 and 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 or 0101 is  {:04b}", 0b0011u32 | 0b0101);
    println!("0011 xor 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is weitten as {}", 1_000_000u32);

}