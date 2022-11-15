#[allow(dead_code)]
pub fn main() {
    let n: u8 = 5;
    match n {
//        守卫
        n if n > 10 => println!(">10"),
        1 => println!("one"),
//        匹配多个值
        0 | 2 | 3 => println!("x"),
//        匹配区间  别名绑定
        y @ 4..=9 => println!("y:{}",y),
        _ => println!("may be ten"),
    }

    let b = true;
    let b1 = match b {
        true => 0,
        false => 1,
    };
    println!("{}", b1);

    let point = (-55, 100);
    let i = match point {
        (x, y) if x > 0 && y > 0 => 1,
        (x, y) if x < 0 && y > 0 => 2,
        (x, y) if x < 0 && y < 0 => 3,
        (x, y) if x > 0 && y < 0 => 4,
        (_, _) => 0
    };
    println!("象限：{}", i)
}
///ilet
pub fn ilet(){
//    let number = Some(6);
    let number:Option<i32> = None;
    let b = true;
    if let Some(i) = number{
        println!("matched:{:?}",i)
    }else if  b {
        println!("abaabaaba")
    }else {
        println!("default")
    }
}

pub fn whilelet(){
    let mut p = Some(0);
    while let Some(i) = p{
        if i>9 {
            println!("quit");
            p = None;
        }else{
            println!("i:{}",i);
            p = Some(i+1);
        }
    }
}