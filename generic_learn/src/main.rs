struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn x(&self) -> i32{
        &self.x + 10
    }
}


fn main() {
    let point = Point{x: 23, y: 33};
    println!("{}", point.x());
}
