use std::fmt;

pub fn t_fmt() {
    let day2 = 32;
    println!("{} days!", day2);
    let a = "Tom";
    let b = "Bobe";
    println!("{0}:hai~{1}  {1}:hai~{0} ", a, b);

    println!("{good}的单价是{rmb}元", good = "烧鸡", rmb = 25);

    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    //fmt::Debug
    #[derive(Debug)]
    struct Point(i32, i32);
    let p1: Point = Point(555, 555);
    println!("{:?}", p1);
    println!("{:#?}", p1);
    println!("{}", p1);
    //fmt::Display
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(X:{},Y:{})", self.0, self.1)
        }
    }
    #[derive(Debug)]
    struct Complex(f32, f32);
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.0, self.1)
        }
    }
    let c = Complex(3.3, 7.2);
    println!("Display:{}", c);
    println!("Debug:{:?}", c);


    let v = List(vec![1,2,3]);
    println!("{}",v)
}


struct List(Vec<i32>);
pub fn fmt_list(){
    impl fmt::Display for List{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           let List(ref vec) = *self;
            write!(f,"[")?;
            for(count,v) in vec.iter().enumerate(){
                if count != 0 {write!(f,",")?;}
                write!(f,"{}:{}",count,v)?;
            }
            write!(f,"]")
        }
    }
}