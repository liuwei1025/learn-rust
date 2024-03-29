# learn-rust
学习Rust

[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)

[中文版](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html)

## 问题

1. 如何打印枚举值?
    > `vector::SpreadsheetCell` doesn't implement `std::fmt::Display`
    ```rust
    enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
    }
    println!("elem is {}", SpreadsheetCell::Int(2))
    ```


## 联系

1. 常见集合
    >给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希函数会很有帮助）。

    ```rust
    ```

    >将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！

    >使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
