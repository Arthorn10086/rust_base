#![allow(dead_code)]

mod std_fmt;
mod primitive_type;
mod struct_t;
mod enum_t;
mod type_tran;
mod flow_control;
mod match_t;
mod fn_t;
mod trait_t;
mod fb;
mod macro_t;
mod panic_t;
mod option_t;
mod option_then;
mod result_t;
mod error_t;
mod box_t;
mod hashmap_t;
mod spawn_t;
mod channel_t;
mod file_t;
mod pro_t;
mod crc;
mod build_crc;
use crc::*;

fn main() {
    std_fmt::t_fmt();
//    primitive_type::ptype();
//    primitive_type::op();
//    primitive_type::t_tuple();
//     struct_t:: ts();
//     struct_t::t_rect_area();
//    struct_t::t_square();
//    enum_t::main();
//    enum_t::main1();
//     type_tran::main();
//    type_tran::main1();
//    flow_control::test_if(100);
//    flow_control::test_loop();
//    flow_control::test_while();
//    flow_control::test_for();
//    match_t::main();
//    match_t::ilet();
//    match_t::whilelet();
//    fn_t::fizzbuzz(100);
//    fn_t::main();
//    trait_t::main();
//    fb::main();
//    macro_t::main();
//    panic_t::main();
//    option_t::main();
//    option_then::main();
//    result_t::main();

//    error_t::main();
//    box_t::main();
//    hashmap_t::main();
//    spawn_t::main();
//    channel_t::main();
//     file_t::main();
//     pro_t::main2();
//    file_t::main2();
//     build_crc::main();
//     let v:Vec<i64> = vec![110,97,109,101];
//     let v1 =v.iter().map(|c|*c as u8).collect::<Vec<_>>();
//     // let v2 = v as &[u8];
//     let v2 = String::from_utf8(v1).unwrap();
//     let v3 = v2.as_bytes();
//     // let mut state = State::<AUG_CCITT>::new();
//     println!("{}",State::<AUG_CCITT>::calculate(v3));


}
