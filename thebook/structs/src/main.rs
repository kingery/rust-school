/*
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
*/

#[derive(Debug)]
struct Rect {
    height: u32,
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn square(size: u32) -> Rect {
        Rect {
            height: size,
            width: size,
        }
    }
}


fn main() {
    let rect = Rect { height: 30, width: 50 };

    println!("Area is {}", rect.area());
    println!("Rectangle is {:#?}", rect);
    println!("{:#?}", Rect::square(5));
}

fn area(rect: Rect) -> u32 {
    rect.height * rect.width
}
    /*
    println!("Hello, world!");
    let user1 = User {
        active: true,
        email: String::from("someemail@email.biz"),
        username: String::from("Gary"),
        sign_in_count: 0,
    };

    println!("email: {}", user1.email);

    let user2 = build_user(String::from("Bob"), String::from("bob@bobnet.bob"));
    println!("user2 name: {}", user2.username);

    let user3 = build_user_init_shorthand(String::from("foo"), String::from("bar"));

    println!("user3 name: {}", user3.username);
}

fn build_user(username:String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_init_shorthand(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    }
}
*/
