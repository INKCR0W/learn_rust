//主函数
fn main() {
    //单行注释
    /* 注释 */
    /*
     * 多行注释 
     */

    //两种输出的方法，println会在字符串后面添加一个换行符，而print不会
    print!("Hello, world!");
    println!("Hello, world!");

    //两个函数都可以使用的占位符{}，占位符里面可以写后面函数的下标，从0开始，输出{或}用{{或}}
    println!("第一个参数是{0}, 第二个参数是{1}, {{}}", 1, 2);

    //这是一个个不可变变量
    let a = 123; //a = 321; 将报错，因为a的值不可改变
    //未初始化的不可变变量可以赋值一次

    //这是一个可变变量，b的值可以被改变
    let mut b = 123;
    b = 321; //合法

    //这是一个常量，同样不可变
    const c: i32 = 123; //c = 321; 一样会报错，常量的值当然不可变
    //rust会自动判断类型，如果没有声明类型，a会被判断为有符号32位整型，也就是 i32

    //rust的重影（Shadowing)概念，可以理解为标识符的重复使用，是用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以发生变化
    let a: &str = "Shadowing Variable";
    let b: i32 = 123;
    //而对c的重影操作将会导致报错，因为c是常量

    //rust会默认把浮点数判断为64位浮点数，因为现代计算机对这两种浮点数的计算速度几乎相同，但64位浮点数精度更高
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    //自运算符号，rust不支持 ++ 和 -- ，因为这两种运算符出现在变量的前后会影响代码的可读性
    let mut b = 1;
    b += 1;

    let a: char = 'a';

    println!("{}", a);

    //函数调用
    another_function(63);

    //rust可以把一个复杂的表达式写在一对{}里
    let d = {
        let a = 5; //这个a是局部临时变量，不会影响外面的a
        a + 20 //最后一行不要写分号才是有返回值的表达式，不然就变成语句了
    };
    //5 + 20 = 25
    println!("{}", d);

    //rust中，函数定义可以嵌套，函数必须指定返回值的数据类型(->)
    fn five() -> i32 {
        5
    }

    println!("{}", add(1, 5));

    let a = 12;
    let b;

    //rust单语句也必须使用{}括起来，if后面的表达式可以不写()，写也没啥问题
    if a > 0 {
        b = 1;
    }
    else if a < 0 {
        b = -1;
    }
    else {
        b = 0;
    }
    println!("b is {}", b);

    //可以用这种方式实现类似三元运算符的效果
    let number = if a > 0 { 1 } else { -1 };
    println!("number is {}", number);

    let mut number = 1;
    //while循环
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("循环结束，number的值是{}", number);

    /* 
    rust没有类似于C语言那种三元的for循环，需要用while循环代替
    
    C
    for (int i = 0; i < 10; i++){}

    rust
    let mut i = 0;
    while i < 10 {
        i += 1;
    }
    */

    //rust的for循环可以用来遍历数组
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为{}", i)
    }

    //使用下标访问
    for i in 0..5 {
        println!("下标{} = {}", i, a[i]);
    }


}

//rust的函数无所谓声明位置，只要定义过就可以调用
//rust的函数参数列表和C一样必须规定数据类型
fn another_function(a: i32) {
    println!("a的值是{}", a);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}