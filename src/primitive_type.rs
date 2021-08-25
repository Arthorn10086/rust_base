use std::fmt;
use std::mem;
#[allow(dead_code)]
pub fn ptype() {
    //标量类型
    let is_true = true;
    //默认
    let i0 = 32;
    //常规声明
    let i1: u32 = 32;
    //后缀声明
    let i2 = 32i64;
    // mut 可变变量 根据上下文自动推断类型
    let mut i3 = 32i64;
    i3 += 1;
    let c = 'a';
    println!("{},{},{},{},{},{}", is_true, i0, i1, i2, i3, c);

    //复合类型
    //数组  一组拥有相同类型 T 的对象的集合，在内存中是连续存储的
    let array = [1, 2, 3];

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);
    println!("xs len:{}", xs.len());
    println!("xs mem:{}", mem::size_of_val(&xs));
    fn slice_show(slice: &[i32]) {
        println!("{:?}", slice);
        println!("slice len:{}", slice.len());
        println!("slice mem:{}", mem::size_of_val(slice))
    }
    slice_show(&xs);
    slice_show(&xs[1..4]);
    //元组  一个可以包含各种类型的值的组合
    let tuple = (1, 1.2, 'a', "a");
    let tuple1 = (('a', 'b'), 1, 2, 3);
    let (t1, x, y, z) = tuple1;
    println!("{:?}", tuple);
    println!("{:?}", tuple1);
    println!("{:?}", (1, ));
    println!("{:?}", (1));
    println!("{:?},{},{},{}", t1,x,y,z);
    //struct
    #[derive(Debug)]
    struct Point(u32, u32);
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({},{})", self.0, self.1)
        }
    }
    println!("{:?}", Point(500, 499));
    println!("{}", Point(500, 499));
}
#[allow(dead_code)]
pub fn op() {
    let u: u32 = 1u32 + 2;
    let i: i32 = 1i32 - 2;
    let a = 1_000_1000u32;
    println!("{},{},{}", u, i, a);
    //位运算
    let x = 0b011u32 & 0b101;
    let y = 0b1101u32 | 0b0010;
    let m = 1 << 2;
    let n = 4 >> 1;
    println!("{:b},{:b},{:b},{:b}", x, y, m, n);
    //短路运算
    let b1 = true && false;
    let b2 = false || true;
    let b3 = tf(false) && tf(true);
    let b4 = tf(true) || tf(false);
    println!("{},{},{},{}", b1, b2,b3,b4);
}

#[allow(dead_code)]
fn tf(a: bool) -> bool {
    println!("input:{}", a);
    return a;
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", &self.0, &self.1, &self.2, &self.3)
    }
}
#[allow(dead_code)]
pub fn t_tuple() {
    let m = Matrix(1.1, 2.2, 3.3, 4.4);
    println!("{}", m)
}