use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
static PANGRAM: &'static str =
    "the quick brown fox jumped over the lazy dog\n";
pub fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }
}

pub fn main2() {

    let mut c = Command::new("escript").arg("D:\\tools\\gongjijin.escript").spawn().unwrap();
    let _r = c.wait().unwrap();
    println!("end,{}",_r);
}