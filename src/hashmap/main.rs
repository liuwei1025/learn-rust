use std::collections::HashMap;

pub fn main() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  for (key, value) in scores.iter() {
    println!("{}: {}", key, value)
  }
  let mut map = HashMap::new();
  let text = "hello world wonderful world";

  for elem in text.split_whitespace() {
    // count 是插入值的可变引用存储
    let count = map.entry(elem).or_insert(0);
    *count += 1;
  }
  println!("{:?}", map)
}
