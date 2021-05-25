pub fn add_to_waitlist() {}
  fn seat_at_table() {}
  pub struct Table {
      pub width: u32,
      height: u32
  }
  impl Table {
      pub fn dimension(&self) -> u32 {
          self.height * self.width
      }
      pub fn new(width: u32, height: u32) -> Table {
          Table {
              width,
              height
          }
      }
  }