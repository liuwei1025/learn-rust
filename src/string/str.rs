pub fn main() {
  // +的函数签名 fn add(self, s: &str) -> String
  let s1 = "Hello ".to_string();
  // 虽然s2是 String类型 但是add被调用时
  // rust使用了解引用强制多态（deref coercion）
  // &s2 变成了 &s2[..]
  let s2 = "world!".to_string();

  let s3 = s1 + &s2;

  println!("{}", s3);
  let s1 = "Hi";
  println!("{}-{}", s1, s2);

  // Rust 的字符串不支持索引 因为使用UTF-8编码 每一个字符的长度是不固定的
  let _len = String::from("Hola").len();
  let h = String::from("Здравствуйте");
  // 字节、标量值和字形簇（最接近人们眼中 字母 的概念）
  let s = &h[0..2];
  println!("{}", s);
}
