trait Action {
    fn eat(&self) -> String {
        String::from("ead fucking bullshit!!!")
    }
    fn drink (&self) -> String {
        String::from("drink pea after eat ") + &self.eat()
    }
}

trait Compare {
   fn biggerThan<T> (v1:T, v2:T) -> i32;
}

struct Dog {

}

struct Cat {

}

impl Action for Dog {
    fn eat(&self) -> String {
        String::from("eat bone")
    }
}

impl Action for Cat {
    fn eat(&self) -> String {
        String::from("eat fish")
    }
}

struct Human {

}

impl Action for Human {

}

fn fuck<T:Action> (item: T) -> String {
    item.eat()
}

// fn main () {
//     let cat = Cat{};
//     let dog = Dog{};
//     let human = Human{};
//     println!("cat {}", fuck(cat));
//     println!("dog {}", fuck(dog));
//     println!("human {}", fuck(human));
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    // let mut index = 0;

    // while (index < (&list).len()) {
    //     if (&list[index] > largest) {
    //         largest = &list[index];
    //     }
    //     index = index + 1;
    // }
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}