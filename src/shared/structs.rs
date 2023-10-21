use std::fmt; // Import `fmt`

#[derive(Debug, Default)]
struct Customer {
    first_name: String,
    last_name: String,
    gender: Gender,
    prefix: String,
    #[allow(dead_code)]
    work: WorkType,
}

#[derive(Debug)]
struct Statement {
    #[allow(dead_code)]
    first_name: String,
}
#[allow(dead_code)]
impl Statement {
    pub fn new(first_name: &str) -> Self {
        Statement {
            first_name: first_name.to_string(),
        }
    }
}

#[derive(Debug, Default)]
enum WorkType {
    #[default]
    Engineer,
    #[allow(dead_code)]
    Doctor,
}
#[allow(dead_code)]
impl fmt::Display for WorkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self)
    }
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}
#[allow(dead_code)]
impl Default for Gender {
    fn default() -> Self {
        return Gender::Male;
    }
}
#[allow(dead_code)]
impl Customer {
    fn get_full_name(&self) -> String {
        return format!(
            "{} {} {} {:?}",
            &self.prefix, &self.first_name, &self.last_name, &self.work
        );
    }

    fn set_prefix(&mut self) {
        match self.gender {
            Gender::Male => self.prefix = String::from("Mr."),
            Gender::Female => self.prefix = String::from("Ms."),
        }
    }
}
#[allow(dead_code)]
pub fn sub_main() {
    let statement = Statement::new("Paritosh");
    println!("{:?}", statement);

    let first_customer = Customer {
        first_name: String::from("Paritosh"),
        last_name: String::from("Baghel"),
        gender: Gender::Male,
        ..Default::default()
    };

    let second_customer = Customer {
        first_name: String::from("Binny"),
        last_name: String::from("Rao"),
        gender: Gender::Female,
        ..Default::default()
    };

    let mut customers = Vec::new();
    customers.push(first_customer); //Now customer is moved into customers vector
    customers.push(second_customer);

    println!("=============== Printing customer information ================");

    // This is how you loop through customer mutable ref
    for customer in &mut customers {
        customer.set_prefix();
        println!(" {}", customer.get_full_name());
    }

    // This is how you loop through customer mutable ref exactly same as above as `iter_mut`
    for customer in customers.iter_mut() {
        customer.set_prefix();
        println!(" {}", customer.get_full_name());
    }

    println!("==============================================================");

    println!("=============== Printing customer information ================");

    // This is how you loop through customer non mutable
    for customer in customers.iter() {
        println!(" {}", customer.get_full_name());
    }

    println!("==============================================================");
}
