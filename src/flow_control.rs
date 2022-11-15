#[allow(dead_code)]
pub fn test_if(score: i8) {
    let r = if score < 60 {
        format!("不及格")
    } else if score < 80 {
        format!("及格")
    } else if score < 90 {
//        if每个分支的返回值类型必须一样
//       "良"
        format!("良")
    } else {
        format!("优秀")
    };
    println!("成绩结果：{}", r)
}

//循环
//continue跳过本次循环继续  break中断退出
#[allow(dead_code)]
pub fn test_loop() {
    let mut count = 0u8;
    println!("打印10以内的所有奇数");
    loop {
        count += 1;
        if count == 5 {
            continue;
        }
        if count % 2 == 1 {
            println!("发现一个奇数：{}", count);
        };
        if count == 10 {
            break;
        };
    }

//    'label标签  处理外层循环
    'loop1: loop {
        println!("进入循环1");
        'loop2: loop {
            println!("进入循环2");
            break 'loop1;
        }
    }
    println!("loop1跳出");
//
    let mut x = 0;
    let reslut = 'loop3: loop {
        x += 1;
        if x == 5 {
            break format!("ok");
        } else {
            let mut y = 0;
            'loop4: loop {
                y += 1;
                if x == y {
                    continue 'loop3;
                } else if y == 5 {
                    break;
                } else {
                    println!("x:{},y:{}", x, y)
                }
            };
        }
    };
    println!("{}", reslut);
}
#[allow(dead_code)]
pub fn test_while() {
    let mut c = 0;
    while c <= 100 {
        c+=1;
        if c % 3 == 0 {
            if c%5==0 {
                println!("fizzbuzz")
            }else{
                println!("fizz")
            }
        } else if c % 5 == 0 {
            println!("buzz")
        } else {
            continue;
        }
    }
}
#[allow(dead_code)]
pub fn test_for(){
    for c in 1..100{
        if c % 3 == 0 {
            if c%5==0 {
                println!("fizzbuzz")
            }else{
                println!("fizz")
            }
        } else if c % 5 == 0 {
            println!("buzz")
        } else {
            continue;
        }
    }
}