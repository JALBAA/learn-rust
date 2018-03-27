/*
 * @Author: zhaoye 
 * @Date: 2018-01-26 18:21:17 
 * @Last Modified by: zhaoye
 * @Last Modified time: 2018-01-26 18:25:59
 */
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn get_user () -> User{
    let uu = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    uu
}

fn main() {
    let mut user1 = get_user();
    user1.email = String::from("www");
    let user2 = User {
        email: String::from("wwerwraaaaa"),
        ..user1
    };
    println!("{}", user1.email);
    println!("{}", user2.email);
}