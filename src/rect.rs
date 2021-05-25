// https://kaisery.github.io/trpl-zh-cn/ch05-01-defining-structs.html

// 计算面积
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32
}
pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// Methods
/**
 * methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object),
 * and their first parameter is always self, which represents the instance of the struct the method is being called on
 */
impl Rectangle {
    // 默认第一个参数是示例
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    // 支持入参
    pub fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.height <= self.height && rect.width <= self.width
    }
    // associated function 这个是一个function 而不是 methods
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}
