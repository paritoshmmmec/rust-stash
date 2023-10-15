#[derive(Debug)]
struct Customer {
    first_name: String,
    last_name: String,
}

impl Customer {
    fn get_full_name(&self) -> String {
        return format!("{0} {1}", &self.first_name, &self.last_name);
    }

    fn set_full_name(&mut self) {
        self.first_name = String::from("Mr. ");
    }
}

pub fn sub_main() {
    let first_customer = Customer {
        first_name: String::from("Paritosh"),
        last_name: String::from("Baghel"),
    };

    let second_customer = Customer {
        first_name: String::from("Binny"),
        last_name: String::from("Rao"),
    };

    let mut customers = Vec::new();
    customers.push(first_customer); //Now customer is moved into customers vector
    customers.push(second_customer);

    println!("=============== Printing customer information ================");

    for customer in &mut customers {
        customer.set_full_name();
        println!(" {}", customer.get_full_name());
    }

    println!("==============================================================");
}
