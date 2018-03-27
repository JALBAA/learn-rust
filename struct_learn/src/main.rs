#[derive(Debug)]
struct Square {
    height: u32,
    width: u32,
}
impl Square {
    fn area(&self, extra: &u32) -> u32 {
        self.height * self.width * extra
    }
}
fn main() {
    let square = Square {
        height: 20,
        width: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        square.area(&20)
    );
    println!("{:#?}", square);
}
