use std::fmt;
#[allow(dead_code)]
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


    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    let city = City { name: "chengdu".to_string(), lat: 3.5, lon: 5.6 };
    println!("{}", city);

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() { // 一旦添加了针对 fmt::Display 的实现，则要用 {} 对输出内容进行转换
         println!("{}", *color) }
}


struct List(Vec<i32>);
#[allow(dead_code)]
pub fn fmt_list() {
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let List(ref vec) = *self;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ",")?; }
                write!(f, "{}:{}", count, v)?;
            }
            write!(f, "]")
        }
    }
}

struct City {
    name: String,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}


struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = &self.red;
        let g = &self.green;
        let b = &self.blue;
        write!(f, "RGB({},{},{}) 0x{:02X}{:02X}{:02X}", r, g, b, r, g, b)
    }
}
