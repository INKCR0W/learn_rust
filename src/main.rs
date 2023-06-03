// 主函数
fn main() {
    // 单行注释
    /* 注释 */
    /*
     * 多行注释 
     */

    // 两种输出的方法，println会在字符串后面添加一个换行符，而print不会
    print!("Hello, world!");
    println!("Hello, world!");

    // 两个函数都可以使用的占位符{}，占位符里面可以写后面函数的下标，从0开始，输出{或}用{{或}}
    println!("第一个参数是{0}, 第二个参数是{1}, {{}}", 1, 2);

    // 这是一个个不可变变量
    let a = 123; // a = 321; 将报错，因为a的值不可改变
    // 未初始化的不可变变量可以赋值一次

    // 这是一个可变变量，b的值可以被改变
    let mut b = 123;
    b = 321; // 合法

    // 这是一个常量，同样不可变
    const c: i32 = 123; // c = 321; 一样会报错，常量的值当然不可变
    // rust会自动判断类型，如果没有声明类型，a会被判断为有符号32位整型，也就是 i32

    // rust的重影(Shadowing)概念，可以理解为标识符的重复使用，是用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以发生变化
    let a: &str = "Shadowing Variable";
    let b: i32 = 123;
    // 而对c的重影操作将会导致报错，因为c是常量

    // rust会默认把浮点数判断为64位浮点数，因为现代计算机对这两种浮点数的计算速度几乎相同，但64位浮点数精度更高
    let x = 2.0; //  f64
    let y: f32 = 3.0; //  f32

    // 自运算符号，rust不支持 ++ 和 -- ，因为这两种运算符出现在变量的前后会影响代码的可读性
    let mut b = 1;
    b += 1;

    let a: char = 'a';

    println!("{}", a);

    // 函数调用
    another_function(63);

    // rust可以把一个复杂的表达式写在一对{}里
    let d = {
        let a = 5; // 这个a是局部临时变量，不会影响外面的a
        a + 20 // 最后一行不要写分号才是有返回值的表达式，不然就变成语句了
    };
    // 5 + 20 = 25
    println!("{}", d);

    // rust中，函数定义可以嵌套，函数必须指定返回值的数据类型(->)
    fn five() -> i32 {
        5
    }

    println!("{}", add(1, 5));

    let a = 12;
    let b;

    // rust单语句也必须使用{}括起来，if后面的表达式可以不写()，写也没啥问题
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

    // 可以用这种方式实现类似三元运算符的效果
    let number = if a > 0 { 1 } else { -1 };
    println!("number is {}", number);

    let mut number = 1;
    // while循环
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

    // rust的for循环可以用来遍历数组
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为{}", i)
    }

    // 使用下标访问
    for i in 0..5 {
        println!("下标{} = {}", i, a[i]);
    }

    // rust提供了一种无限循环结构 loop
    let s = ['C', 'R', 'O', 'W', 'L', 'O', 'V', 'E'];
    let mut i = 0;
    println!("L的位置是{}", loop {
        let ch = s[i];
        if ch == 'L' {
            // rust的break可以返回值
            break i;
        }
        println!("\'{}\'", ch);
        i += 1;
    });

   // 变量的生命周期
   {
    // 在声明以前，变量 s 无效
    let s = "some shit";
    // 这里是变量 s 的可用范围
   }
   // 变量 s 无效


   // 变量的复制
   let x = 5;
   let y = x;
   // 这个时候有两个“5”， 因为此处 x 是基本类型 i32 ，是在栈内的，rust会进行深度复制

   let s1 = String::from("hello");
   let s2 = s1;
   // 这个时候实际上只有一个hello，rust进行浅复制，仅仅是把 s2 指向原本的那个 "hello"，同时，为了防止变量生命周期结束的时候释放两次，s1 会被释放

   // 这样才是深复制
   let s1 = String::from("hello");
   let s2 = s1.clone();
   println!("s1 = {}, s2 = {}", s1, s2);

   // 函数传参也是一样，如果没有返回值的话，原本的s1就会被释放
   takes_ownership(s1);
   // s1 不可用了

   // 基本类型不会被释放
   let x = 5;
   makes_copy(x);
   // x 依旧可用

   // rust变量的“租借”概念
   let s1 = String::from("hello");
   // 引用，很多高级语言里都有的概念(大概可以理解为指针)，在rust中，引用会租借(Borrow)值的所有权
   let mut s2 = &s1;
   println!("s2 的长度是{}", s2.len()); // 可以访问租借的值
   let mut s3 = s1;
   // 这个时候 s2 不可用，因为 s2 租借的 s1 的值的所有权已经从 s1 移动到了 s3，所以需要重新租借(从s3)
   s2 = &s3;
   println!("{}", s2);
   // btw， 租借的值不可修改
   // s2.push_str("crow"); 将会报错
   // 但是如果是可变引用，就可以改变租借的值的内容了，注意可变引用的对象也必须是可变的，所以上面我使用了 let mut 声明 s3
   let s2 = &mut s3;
   s2.push_str("crow");
   println!("修改后的租借的值是{}", s2);

   // rust中一个值一次只可被一个变量可变引用
   // let s4 = &mut s3; 将会报错
   // 可以被多个变量不可变引用
   let s4 = &s3; // 没有问题

   
   // 切片(Slice)类型，
   let s = String::from("string");
   
}

// rust的函数无所谓声明位置，只要定义过就可以调用
// rust的函数参数列表和C一样必须规定数据类型
fn another_function(a: i32) {
    println!("a的值是{}", a);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// rust不允许出现垂直引用(Dangling References)，可以理解为野指针或空指针，如果出现这种变量，rust将在编译阶段发现并阻止，所以说下面这个函数将会导致报错
// 这个doc里我没有调用这个函数所以它会被忽略，如果调用的话的确是会报错的
fn dangle() -> &String {
    let s = String::from("hello");

    // s的生命周期已经结束
    return &s;
}