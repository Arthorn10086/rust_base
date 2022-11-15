#![allow(dead_code)]

struct Dog { name: &'static str, breed: &'static str }

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn action(&self) {
        println!("行动")
    }
}

impl Dog {
    fn new(s1: &'static str, s2: &'static str) -> Dog {
        Dog { name: s1, breed: s2 }
    }
}

impl Animal for Dog {
    fn new(s: &'static str) -> Dog {
        Dog { name: s, breed: "未知" }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn action(&self)  {
        println!("跑起来")
    }
}


// `Centimeters`，可以比较的元组结构体
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);
// `Inches`，可以打印的元组结构体
#[derive(Debug)]
struct Inches(i32);
impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}
// `Seconds`，不带附加属性的元组结构体
struct Seconds(i32);




struct Droppable {
    name: &'static str,
}
// 这个简单的 `drop` 实现添加了打印到控制台的功能。
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}



pub fn main() {
    let mut dog: Dog = Animal::new("大白");
    println!("name:{}",dog.name());
    dog.action();

    let _one_second = Seconds(1);
    // 报错：`Seconds` 不能打印；它没有实现 `Debug` trait
    //println!("One second looks like: {:?}", _one_second);
    // 试一试 ^ 将此行注释去掉
    // 报错：`Seconds`不能比较；它没有实现 `PartialEq` trait
    //let _this_is_true = (_one_second == _one_second);
    // 试一试 ^ 将此行注释去掉
    let foot = Inches(40);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} than one meter.", cmp);





    let _a = Droppable { name: "a" };
    // 代码块 A
    {
        let _b = Droppable { name: "b" };
        // 代码块 B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }
    println!("Just exited block A");
    // 变量可以手动使用 `drop` 函数来销毁。
    drop(_a);
    // 试一试 ^ 将此行注释掉。
    println!("end of the main function");
    // `_a` **不会**在这里再次销毁，因为它已经被（手动）销毁。
}