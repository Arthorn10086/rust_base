// 隐藏未使用代码警告的属性。
#![allow(dead_code)]

#[derive(Debug)]
struct EBook {
    url: String,
    btype: i32,
}

#[derive(Debug)]
struct PBook {
    author: String,
    press: String,
}

#[derive(Debug)]
enum Book {
    //禁书
    Obscene,
    //电子书
    EBook(EBook),
    //纸质书
    PBook(PBook),
}

// 此函数将一个 `Book` enum 作为参数，无返回值。
fn inspect(b: Book) {
    // `enum` 的使用必须覆盖所有情形（无可辩驳的），所以使用 `match`以分支方式覆盖所有类型。
    match b {
        Book::Obscene => println!("Is Obscene!"),
        Book::EBook(e) => println!("{:?}", e),
        Book::PBook(p) => println!("{:?}", p)
    }
}

enum Fighter {
    Attack,
    Flee,
    Restore(u8, u32),
    Skill(u16),
}

use Fighter::*;

fn operation(f: Fighter) {
    match f {
        Attack => println!("进攻"),
        Flee => println!("溜了溜了"),
        Restore(t, v) =>
            match t {
                1 => println!("回复血量值{}", v),
                2 => println!("回复法力值{}", v),
                _ => println!("input_error"),
            },
        Skill(sid) => println!("释放技能{}", sid),
    }
}


pub fn main() {
    // `to_owned()` 从一个字符串 slice 创建一个具有所有权的 `String`。
    let pbook = Book::PBook(PBook { author: "Bob".to_owned(), press: "鬼话连篇出版社".to_owned() });
    let ebook = Book::EBook(EBook { url: "www.qidian.com".to_owned(), btype: 2 });
    let obscene = Book::Obscene;
    inspect(pbook);
    inspect(ebook);
    inspect(obscene);

    let op1 = Attack;
    let op2 = Flee;
    let op3 = Restore(2u8, 10_000u32);
    let op4 = Skill(23u16);
    operation(op1);
    operation(op2);
    operation(op3);
    operation(op4);
}

//实现单链表
enum List {
    // Cons： 元组结构体，包含一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil： 末结点，表明链表结束
    Nil,
}

use List::*;

impl List {
    //创建一个空链表
    fn new() -> List {
        Nil
    }
    //追加
    fn append(self, value: u32) -> List {
        Cons(value, Box::new(self))
    }
    //长度
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    fn tostring(&self) -> String {
        match *self {
            Cons(v, ref tail) => {
                format!("{},{}", v, tail.tostring())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}


pub fn main1() {
    let mut l = List::new();
    l = l.append(1);
    l = l.append(2);
    println!("len:{}", l.len());
    println!("str:{}",l.tostring());
}
