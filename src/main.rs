struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("anotoher@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let user3 = build_user(String::from("hoge@example.com"), String::from("hoge"));

    user1.email = String::from("anothermail@example.com");
    println!("{}", user3.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}