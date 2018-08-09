// 在所有的作用域外声明全局变量。
static mut LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在 main 函数(主函数)中访问常量
    unsafe {
      println!("This is {}", LANGUAGE);
    }
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    unsafe {
      LANGUAGE = "jfdisjfofdsf";
    }
    // 报错！不能修改一个 `const` 常量。
    // THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}