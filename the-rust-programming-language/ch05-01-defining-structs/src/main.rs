fn main() {
    let mut u1 = User {
        active: true,
        username: String::from("dp"),
        email: String::from("dp@a.com"),
        sign_in_count: 1,
    };
    u1.email = String::from("dp@b.com");

    // let u2 = User {
    //     active: u1.active,
    //     username: u1.username,
    //     email: String::from("dp@c.com"),
    //     sign_in_count: u1.sign_in_count,
    // };

    // 简写
    let _u2 = User {
        email: String::from("dp@c.com"),
        ..u1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}

#[allow(unused)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn _build_user(email: String, username: String) -> User {
    User {
        active: true,
        // username: username, // 可以简写
        username,
        // email: email,
        email,
        sign_in_count: 1,
    }
}

// 如果我们在结构体的字段定义上使用了引用，那么需要引入生命周期标记
struct _User2 {
    // name: &str, // 我们无法确保 _User2 的字段 name 是有效的，因为它是一个引用
    name: String, // String 是一个拥有所有权的类型。
}

// 元组结构体
#[allow(unused)]
struct Color(i32, i32, i32);

#[allow(unused)]
struct Point(i32, i32, i32);

// 没有任何字段的类单元结构体
struct AlwaysEqual;
