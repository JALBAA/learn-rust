#[derive(Debug)]
enum ETest{
    V1,
    V2 (i32, i32),
    V3 {x:i32, y:i32},// 匿名结构体
}

fn main() {
    let a = ETest::V2(2, 3);
    let b = Some(4);
    println!("{:?}", a);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
