fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    x
}
fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("abcd");
        result = longest(&string1[0..], string2.as_str());
    }

    println!("The longest string is {}", result);
}