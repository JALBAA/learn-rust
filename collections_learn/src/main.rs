// fn test<T>(sth: Vec<T>) -> Result <Vec<T>, &'static str> {
//     Ok(sth)
// }
use std::error::Error;

// fn main() {
//     let sth = test(vec!["1", "2", "3434"]).unwrap();
//     println!("{:?}", sth);
//     let aa = sth.get(1111);
//     match &aa {
//         &None => {
//             println!("{}", 333);
//         },
//         _ => {
//             println!("{:?}", 111);
//         },
//     }
// }
fn test () -> Result<i32, &'static str> {
    Ok(234)
}
fn main () {
    let a = test();
    match a {
        Ok(123) => {
            println!("{}", 123);
        },
        _ => {
            println!("{}", "error");
        },
    }
}