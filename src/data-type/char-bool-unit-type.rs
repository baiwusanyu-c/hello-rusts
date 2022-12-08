
// 字符类型
fn charType(){
    // 字符类型使用单引号来定义 '',且只有一个字母
    const charTest:char = 'c';
    println!("{}", charTest);
}

// 布尔类型
fn boolType(){
    // 字符类型使用单引号来定义 '',且只有一个字母
    const falseVar:bool = false;
    const trueVar:bool = true;
    println!("{}", charTest);
}

// 单元类型
fn unitType(){
   // 单元类型就是 () ，对，你没看错，就是 () ，唯一的值也是 ()
   // 没错， main 函数就返回这个单元类型 ()，你不能说 main 函数无返回值，
   // 因为没有返回值的函数在 Rust 中是有单独的定义的：发散函数( diverge function )，顾名思义，无法收敛的函数。
}

// 函数没有返回值，那么返回一个 () 单元类型
// 通过 ; 结尾的表达式返回一个 () 单元类型
// 在实际编程中，你会经常在错误提示中看到该 () 的身影出没，
// 假如你的函数需要返回一个 u32 值，但是如果你不幸的以 表达式; 的方式作为函数的最后一行代码，就会报错：
// fn add(x:u32,y:u32) -> u32 {
//     x + y;
// }