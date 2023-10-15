#[allow(dead_code)]
pub fn print_me(value: &str) {
    println!("Print input value:{value}")
}

#[allow(dead_code)]
pub fn sub_main() {
    let welcome: &str = "Hello World"; //Note the type of variable is &str

    print_me(welcome); //This works
    print_me(&welcome); //This works too however redundant as reference of slice is slice. :)
}
