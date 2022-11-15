use std::mem;
#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
fn boxed_origin() -> Box<Point> {
    // 在堆上分配这个点（point），并返回一个指向它的指针
    Box::new(Point { x: 0.0, y: 0.0 })
}
pub fn main() {
    // （所有的类型标注都是可要可不要的）
    // 栈分配的变量
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    };
    // 堆分配的 rectangle（矩形）
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin()
    });
    // 函数的输出可以装箱（boxed）
    let boxed_point: Box<Point> = Box::new(origin());
    // 双重间接装箱（Double indirection）
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());
    println!("Point occupies {} bytes in the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in the stack",
             mem::size_of_val(&rectangle));
    // box 的大小 = 指针 大小（box size = pointer size）
    println!("Boxed point occupies {} bytes in the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes in the stack",
             mem::size_of_val(&box_in_a_box));
    // 将包含在 `boxed_point` 的数据复制到 `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             mem::size_of_val(&unboxed_point));

    // 迭代器可以收集到 vector
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);
    // `vec!` 宏可用来初始化一个 vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);
    // 在 vector 的尾部插入一个新的元素
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);
    // 报错！不可变 vector 不可增长
//    collected_iterator.push(0);
    // 改正 ^ 将此行注释掉
    // `len` 方法获得一个 vector 的当前大小
    println!("Vector size: {}", xs.len());
    // 在中括号上加索引（索引从 0 开始）
    println!("Second element: {}", xs[1]);
    // `pop` 移除 vector 的最后一个元素并将它返回
    println!("Pop last element: {:?}", xs.pop());
    // 超出索引范围将抛出一个 panic
//    println!("Fourth element: {}", xs[3]);

    // （所有的类型标注都是都是多余）
    // 一个指向在只读内存中堆分配字符串的引用
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);
    // 逆序迭代单词，不用分配新的字符串
    // （原文：Iterate over words in reverse, no new string is allocated）
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }
    // 复制字符到一个 vector，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    // 创建一个空的且可增长的 `String`
    let mut string = String::new();
    for c in chars {
        // 在字符串的尾部插入一个字符
        string.push(c);
        // 在字符串尾部插入一个字符串
        string.push_str(", ");
    }
    println!("String: {}", string);
    // 此切割的字符串是原字符串的一个切片，所以没有执行新分配操作
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);
    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
    println!("ln: {}", 0.1_f64.ln());
    checked::op(1.0, 10.0);
}



mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }
    type MathResult = Result<f64, MathError>;
    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }
    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }
    fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
    // 中间函数
    fn op_(x: f64, y: f64) -> MathResult {
        // 如果 `div` “失败”了，那么 `DivisionByZero` 将被返回
        let ratio = div(x, y)?;
        // 如果 `ln` “失败”了，那么 `NegativeLogarithm` 将被返回
        let ln = ln(ratio)?;
        sqrt(ln)
    }
    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(match why {
                MathError::NegativeLogarithm
                => "logarithm of negative number",
                MathError::DivisionByZero
                => "division by zero",
                MathError::NegativeSquareRoot
                => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}