#[derive(Debug)]
pub struct Item {
    id: i64,
    name: String,
    sku: String,
}

impl Item {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_sku(&mut self, value: String) {
        self.sku = value;
    }
}

pub fn sub_main() {
    let mut rug_item = Item {
        id: 1,
        name: String::from("Rug"),
        sku: String::from("SKU013838"),
    };
    rug_item.set_sku(String::from("Hello"));
}
