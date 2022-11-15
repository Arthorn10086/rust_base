use std::error::Error;
use std::io;
use std::fs::*;
use std::fs;
use std::io::prelude::*;
use std::path::Path;


pub fn main() {

    let path = Path::new("C:\\");
    let p1 = path.join("Users").join("Public").join("内外网").join("a.txt");
    let d = p1.display();

    let mut file = match File::open(&p1) {
        Err(e) => panic!("error {},{}", d, e.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    let s = match file.read_to_string(&mut s) {
        Ok(_) => s,
        Err(e) => panic!("error {},{}", d, e.description())
    };
    println!("content:{:?}",s);
    let p2 = Path::new("D:\\b.txt");
    let d2 = p2.display();
    let mut f2 = File::create(&p2).unwrap();
    let bytes = s.into_bytes();
    f2.write_all(&bytes);
}


pub fn main2(){
    fs::create_dir("test");
    println!("echo hello > test/test.txt");
    echo(&Path::new("test/test.txt"),"hello").unwrap_or_else(|e|{println!("{:?}",e.kind())});

    match cat(&Path::new("test/test.txt")){
        Err(e)=> println!("cat err:{:?}",e.kind()),
        Ok(s)=>println!("content:{:?}",s),
    }
}

fn cat (path:&Path)->io::Result<String>{
    let mut f = File::open(path)?;
    let mut s = String ::new();
    match f.read_to_string(&mut s){
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }
}

fn echo(path:&Path,s:&str)->io::Result<()>{
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

