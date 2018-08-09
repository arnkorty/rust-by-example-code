fn main() {
    let mut _mutable_integer = 7i32;

    {
        // 借用 `_mutable_integer`
        let _large_integer = &_mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        // _mutable_integer = 50;
        // 改正 ^ 注释掉此行

        // `_large_integer` 离开作用域
    }

    // 正常运行！`_mutable_integer` 在这作用域没有冻结
    _mutable_integer = 3;
}
