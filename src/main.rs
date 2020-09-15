mod phrases;

use phrases::chinese;

struct User {
    username: String,
    email: String
}
// tuple structs
struct Color (u8, u32, u32);
struct Point (i32, i32, i32);

struct Group {
    user: User
}

fn main() {
    let username = "xiaoyu";
    let user1 = set_username(username.to_string());
    println!("Hello, {}!", user1.username);
    let user2 = User {
        username: String::from("liuwei"),
        ..(user1)
    };
    let group1 = Group{
        user: user2
    };
    println!("Hello {}", group1.user.email);
    // tuple structs
    let color = Color(255, 0, 0);
    let point = Point(-1, -1, -1);
    println!("Color is {}", color.0);
    println!("Point is {}", point.0);

    let rect1 = Rectangle{
        width: 10,
        height: 20
    };
    println!("THe rect1 is {:?}", rect1);
    println!("THe rect1 is {:#?}", rect1);
    println!("The area(rect1) is {}", area(&rect1));
    println!("The rect1.area() is {}", rect1.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    chinese::farewells::everyone_will_know_you()
}

fn set_username(username: String) -> User {
    User {
        email: String::from("we1025@qq.com"),
        username
    }
}
// 计算面积
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// Methods
/**
 * methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object), 
 * and their first parameter is always self, which represents the instance of the struct the method is being called on
 */
impl Rectangle {
    // 默认第一个参数是示例
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 支持入参
    fn can_hold(&self, rect: &Rectangle) -> bool {
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
