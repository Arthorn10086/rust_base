#![allow(dead_code)]
//元组结构体
#[derive(Debug)]
struct Point(i32, i32);

// `#[allow(dead_code)]` 属性可以抑制 `dead_code` lint
#[allow(dead_code)]
#[derive(Debug)]
pub struct Line {
    start_point: Point,
    end_point: Point,
}
#[allow(dead_code)]
pub fn ts() {
    let point = Point(22, 1);
    println!("{},{}", point.0, point.1);
//    解构
    let Point(x, y) = point;
    println!("{},{}", x, y);
    let l = Line {
        start_point: point,
        end_point: Point(0, 0),
    };
    println!("{:?}", l)
}


// 动手试一试：
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// 增加一个计算长方形面积的函数 rect_area（尝试使用嵌套的解构方式）。
fn rect_area(rectangle: Rectangle) -> i32 {
    let Rectangle { p1: Point(x1, y1), p2: Point(x2, y2) } = rectangle;
    return ((x1 - x2) * (y1 - y2)).abs();
}
#[allow(dead_code)]
pub fn t_rect_area() {
    let r = Rectangle { p1: Point(1, 4), p2: Point(2, 8) };
    println!("{}", rect_area(r));
}

// 增加一个函数 square，接受的参数是一个 Point 和一个 i32，并返回一个 Rectangle（长方形）的信息，包括左下角的点，以及长和宽。
fn square(p: Point, n: i32) -> (Rectangle, i32) {
    let Point(x, y) = p;
    let r: Rectangle = Rectangle { p1: Point(x - n, y - n), p2: p };
    return (r, n);
}

pub fn t_square() {
    let p = Point(5, 5);
    let (r, n) = square(p, 3);
    println!("result:{:?},边长是{}", r, n);
}