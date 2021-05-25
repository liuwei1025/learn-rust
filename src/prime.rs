pub fn is_prime(num: i32) -> bool {
  let mut res = num < 2;
  let mut suqare_root = 1;
  while suqare_root * suqare_root < num {
    suqare_root = suqare_root + 1;
  }
  println!("the suqare_root is {}", &suqare_root);
  for i in 2..suqare_root {
    if num % i == 0 {
      res = false;
      break;
    }
    res = true;
  }
  res
}
