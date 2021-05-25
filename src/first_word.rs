// String长度可变 str是不可变长度的string
/**
 * https://kaisery.github.io/trpl-zh-cn/ch04-03-slices.html#%E5%AD%97%E7%AC%A6%E4%B8%B2-slice
 */
pub fn first_word(s: &String) -> &str {
  for (i, &item) in s.as_bytes().iter().enumerate() {
    if item == b' ' {
      return &s[0..i]
    }
  }
  &s[..]
}
