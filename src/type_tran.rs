//消除会溢出的类型转换的所有警告
#![allow(overflowing_literals)]
#[allow(dead_code)]
pub fn main() {
    let f = 1.1f64;
    let i = f as u8;
    let c = 48 as char;
    println!("type_tran {:?},{:?},{:?}", f, i, c);

    println!("1000 as u16:{}",1000 as u16);
//    转无符号类型，加上或减去Max+1,直到值符合类型
    println!("1000 as u8:{}",1000 as u8);
//    转有符号类型，上述操作后去补码
    println!("1000 as i8:{}",1000 as i8);
}
use std::mem;
//字面量
pub fn main1(){
    let x = 2u32;
    let y = 2i8;
    let z = 1f32;

    let i = 2;
    let f = 1.0;

    println!("x:{}",mem::size_of_val(&x));
    println!("y:{}",mem::size_of_val(&y));
    println!("z:{}",mem::size_of_val(&z));
    println!("i:{}",mem::size_of_val(&i));
    println!("f:{}",mem::size_of_val(&f));
}