//let i = 11; //let 不能用于全局
const MAX_OPTION: u32 = 1000; //const 能用于全局，但必须指定类型

fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("x is : {}", x);
    //x = 6; x 默认是不可变的！
    let mut x = 10; //x 能被重新赋值，上面的x将被隐藏
    println!("x is : {}", x);
    x = 11;
    println!("x is : {}",  x);
    println!("MAX_OPTIONS is : {}", MAX_OPTION);

    let spaces = "   ";
    let spaces = spaces.len(); //重新使用let可以与上面不同类型

    let x = 12;
     //x = "hello" //x只能是与上面同类型的

    let tup: (i32, f64, u8) = (500, 6.4, 1); //元组
    println!("x is : {}", tup.1);

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    //表达式和语句是不同的，表达式有返回值，而语句没有返回值，语句后必须分号，表达式后面不需要分号
    let result =my_func();
    let x = 10;

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("counter is 20: {}", result);
}

fn my_func() -> i32 {
    6
}
