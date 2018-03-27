#[derive(Debug)]
enum People {
    white,
    black,
    yellow,
}


enum Animal {
    bird,
    dog,
    cat,
    human(People),
}
enum Animal2 {
    bird,
    dog,
    cat,
}
use People::white;

fn main() {
    let result = match_test(&Animal::bird);
    println!("{}", result);

    let result = match_test(&Animal::dog);
    println!("{}", result);

    let result = match_test(&Animal::cat);
    println!("{}", result);

    let result = match_test(&Animal::human(People::black));
    let result = match_test(&Animal::human(People::yellow));
    let result = match_test(&Animal::human(People::white));
    
    println!("{}", result);


    let result = match_test2(Some(&Animal2::bird)).unwrap();
    println!("{}", result);

    if let Some(People::white) = Some(white) {
        println!("eeee");
    }
}

fn match_test2 (ani: Option<&Animal2>) -> Option<i32> {
    match ani {
        Some(&Animal2::bird) => Some(12),
        Some(&Animal2::dog) => Some(12),
        Some(&Animal2::cat) => Some(12),
        None => None,
    }
}

fn match_test (ani: &Animal) -> i32 {
    match Some(ani) {
        Some(&Animal::bird) => 1,
        Some(&Animal::dog) => 2,
        Some(&Animal::cat) => 3,
        Some(&Animal::human (ref people)) => {
            println!("{:?}", people);
            4
        },
        None => 5,
    }
}