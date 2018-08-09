fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // 生成错误1
    2 * first.parse::<i32>().unwrap() // 生成错误2
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    
    println!("The first doubled is {}", double_first(empty));
    // 错误1：输入 vector 为空
    
    println!("The first doubled is {}", double_first(strings));
    // 错误2：此元素不能解析成数字
}
