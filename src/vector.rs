// vector 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值
// 只能储存相同类型的值
pub fn main() {
  let _v: Vec<i32> = Vec::new();

  let mut v1 = vec![100, 2, 3];
  // 一旦
  let _second = &v1[0];

  println!("_second {}", _second);

  v1.push(1);
  // 此处不能再引用_second
  // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况
  println!("v[0],{}", v1[0]);

  match v1.get(0) {
    Some(first) => println!("the first element is {}", first),
    None => (),
  }

  for elem in &mut v1 {
    // help: `+=` can be used on '{integer}', you can dereference `elem`: `*elem`
    *elem += 50
  }
  println!("v[0],{}", v1[0]);
  // 存储不同类型的值
  let _row = vec![SpreadsheetCell::Int(2), SpreadsheetCell::Float(0.12)];
}

enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}
