use std::fmt::Debug;
use std::fmt::Display;

fn sub_main() {
    let continental_string = "녕_ñ";
    let continental_string_len = continental_string.len();
    println!("Length {continental_string_len}");

    let japense_len = continental_string.chars().count();

    for string_char in continental_string.chars().into_iter() {
        let string_char_len = string_char.len_utf8();
        println!("Char: {string_char} size {string_char_len}");
    }
    println!("Number of charaters {japense_len}");
    // print_hello_world_example();
    // print_hello_world_example_v2();
    // print_hello_world_example_v3();
    compare_me("Hello World", 'a', 'a');
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];

    println!("{:?}, {:?}", take_fifth(new_vec).unwrap(), take_fifth(bigger_vec));
}

fn compare_me<T, U>(statement_v2: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    if num_1 > num_2 {
        println!("variant 1 is better than variant 2 {statement_v2}")
    } else if num_1 == num_2 {
        println!("variant 1 is equal to variant 2 {statement_v2}")
    } else {
        println!("variant 2 is better than variant 1")
    }
}

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        // .len() gives the length of the vec.
        // It must be at least 5.
        None
    } else {
        Some(value[4])
    }
}
fn print_me<T: Debug>(value: T) {
    println!("{:?}", value);
}

fn print_hello_world_example_v2() {
    let mut hello = String::from("Hello");
    add_world_v2(&mut hello);
    println!("{hello}")
}

fn add_world_v2(value: &mut String) {
    value.push_str(" World!");
}

fn print_hello_world_example_v3() {
    let mut hello: &str = "Hello";
    add_world_v3(&mut hello);
    println!("{hello}")
}

fn add_world_v3(value: &mut &str) {
    *value = "Hello World!";
}

fn print_hello_world_example() {
    let hello = String::from("Hello");
    let hello_world = add_world(hello);
    println!("{hello_world}")
}

fn add_world(mut value: String) -> String {
    value.push_str(" World!");
    value
}
