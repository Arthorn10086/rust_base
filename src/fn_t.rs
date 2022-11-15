//函数
pub fn fizzbuzz(num: u32) {
    for num in 1..=num {
        if division(num, 15) {
            println!("fizzbuzz");
        } else if division(num, 5) {
            println!("buzz");
        } else if division(num, 3) {
            println!("fizz")
        } else { continue; }
    }
}

fn division(n: u32, m: u32) -> bool {
    n % m == 0
}


//方法
struct Rectangle {
    l: u8,
    w: u8,
}

impl Rectangle {
    //    静态方法
    fn new(l: u8, w: u8) -> Rectangle {
        Rectangle { l: l, w: w }
    }
    //    实例方法  &self不可变借用，所有权不会moved，对数据只读不能修改
    fn area(&self) -> u8 {
        let Rectangle { l: l, w: w } = self;
        return l * w;
    }
    //    这个方法要求调用者的对象是可变的   &mut self 可变借用，同一时刻最多只有一个可变借用
    fn set_l(&mut self, i: u8) {
        self.l = i;
    }
    fn set_w(self: &mut Self, i: u8) {
        self.w = i;
    }
}

struct Point(Box<i32>, Box<i32>);

impl Point {
    //    值传参，变量的所有权会moved带这个代码块，作用域只在这个代码块，执行完后释放
    fn desory(self) {
        let Point(x, y) = self;
        println!("destory:{},{}", x, y)
    }
}

fn bibao1(){
    //闭包 类型标注和{}都是可选
    let double = |i| i * 2;
    let inc = |i: u32| -> u32{ i + 1 };
    println!("闭包1：{}", double(5));
    println!("闭包2：{}", inc(5));
//    捕获
    let s = "haha";
    let p = ||println!("{}",s);
    p();
    let s = "heihei";
    p();
    let mut i = 1;
    let mut add = |num:i32|{
        i +=num;
        println!("i:{}",i);
    };
    add(3);
//    闭包借用了i,闭包绑定到了add变量，在离开add作用域之前借用不会归还，所以在这里 i=10不能正常执行，等到86行，闭包被销毁，就能正常执行
//    i = 10;
    add(4);
    println!("i:{}",i);
    i= 20;
    println!("i:{}",i);
}

fn apply<F>(f:F) where F:FnOnce(){
    f();
}
fn apply1<F>(f:F) ->i32 where F:Fn(i32)->i32{
    f(3)
}
fn bibao2(){
    let s = "s1s1s1";
    let mut f = "sbsbsb".to_owned();
    let d = ||{
        println!("s:{}",s);
        f.push_str("!");
        println!("f:{}",f);
        std::mem::drop(f);
    };
    apply(d);
    let b = |x|x*2;
    println!("3*2={}",apply1(b));
}

pub fn main() {
    let mut r = Rectangle::new(4, 5);
    println!("area:{}", r.area());
    r.set_l(6);
    r.set_w(8);
    println!("area:{}", r.area());
    let p = Point(Box::new(1), Box::new(2));
    p.desory();
//    p.desory();
    bibao1();
    bibao2();

    let v = vec![1,2,3];
    println!("2 in vec:{}",v.iter().any(|&x|x==2));


}