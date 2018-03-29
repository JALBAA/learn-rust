enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
use List::{Cons, Nil};

fn main () {
     let list = Cons("e",
        Box::new(Cons("b",
            Box::new(Cons("a",
                Box::new(Nil))))));
    

}