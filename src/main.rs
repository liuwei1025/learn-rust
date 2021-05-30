mod phrases;
mod prime;
mod rect;
mod first_word;

mod vector;

use phrases::chinese;
use prime::is_prime;
use rect::{Rectangle, area};
use first_word::first_word;
use ferris_says::say;
use std::io::{stdout, BufWriter};

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
    let username = String::from("xiaoyu ai liuwei");
    let name = first_word(&username);
    println!("first_name: {}", name);
    // let mut writer = BufWriter::new(stdout.lock());
    // say(name.to_string().as_bytes(), name.len(), writer);
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

    chinese::farewells::everyone_will_know_you();

    let bol = is_prime(26);
    println!("is prime?, {}", bol);
}

fn set_username(username: String) -> User {
    User {
        email: String::from("we1025@qq.com"),
        username
    }
}

