macro_rules! add {
    ($a:expr,$b:expr)=> { $a+$b};
    ($a:expr,$b:expr,$t:ty)=>{$a as $t + $b as $t};
    ($a:expr,$($b:tt)*)=>{$a+add!($($b)*)}
}
// `test!` 将以不同的方式来比较 `$left` 和 `$right`，
// 根据所调用的情况确定。
macro_rules! test {
    // 参数不需要使用逗号隔开。
    // 可以使用任意模板（原文：Any template can be used!）！
    ($left:expr; and $right:expr) => (
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    );
    // ^ 每个分支都必须以分号结束。
    ($left:expr; or $right:expr) => (
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    );
}

macro_rules! findmin {
    ($x:expr)=>{$x};
    ($x:expr,$($y:expr),+)=>{
        std::cmp::min($x,findmin!($($y),+))
    }
}

macro_rules! findmax {
    ($x:expr)=>{$x};
    ($x:expr,$($y:expr)*)=>{
        std::cmp::max($x,findmax!($($y)*))
    }
}


pub fn main() {
    println!("reply:{:?}", add!(1,2));
    println!("reply:{:?}", add!(1,2,f64));
    println!("reply:{:?}", add!(1,2,3,4,5));
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
    println!("min:{:?}", findmin!(1,2,3,4));
    println!("max:{:?}", findmax!(2,5));
}









use std::ops::{Add, Mul, Sub};
macro_rules! assert_equal_len {
    // `tt` （token tree，令牌树）指示符用于运算符和令牌。
    // （原文：The `tt` (token tree) designator is used for
    // operators and tokens.）
    ($a:ident, $b: ident, $func:ident, $op:tt) => (
        assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    )
}
macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => (
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    )
}
// 实现 `add_assign`、`mul_assign` 和 `sub_assign` 等函数。
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);
mod test {
    use std::iter;
    macro_rules! test {
        ($func: ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();
                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        }
    }
    // 测试 `add_assign`、`mul_assign` 和 `sub_assign`
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}

