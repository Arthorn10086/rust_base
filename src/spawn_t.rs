use std::thread;

static NUM:i32 = 10;

pub fn main(){
    let mut childs = vec![];
    for i in 0..NUM{
        childs.push(thread::spawn(move ||{println!("{}",i)}));
    }
    for child in childs{
        let _=child.join();
    }
}