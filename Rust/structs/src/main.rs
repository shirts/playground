struct User {
    username: String,
    admin: bool,
    sign_in_count: u64
}


fn main() {
    let mut lychee = build_user(String::from("k00lk@t"));
    println!("sign in count: {}", lychee.sign_in_count);
    lychee = sign_in(lychee);
    println!("sign in count: {}", lychee.sign_in_count);
    println!("{} is an admin?: {}", lychee.username, is_admin(&lychee));
}


fn sign_in(mut user: User) -> User {
    user.sign_in_count += 1;
    user
}

fn build_user(username: String) -> User {
    User {
        username,
        admin: false,
        sign_in_count: 0
    }
}

fn is_admin(user: &User) -> bool {
    user.admin
}
