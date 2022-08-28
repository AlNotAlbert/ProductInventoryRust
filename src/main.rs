use std::string;

fn main() {
    println!("Hello, world!");
}


struct Product {
    price: f64,
    id: i32,
    quantity: i32,
    name: String,
}

impl Product {
    pub fn new(price: f64, id: i32, quantity: i32, name: String) -> Self {
        Self { price, id, quantity, name }
    }
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn product_list(&self) -> Vec<String> {
        // todo!(" return a list of avaliable products");
    let mut products_list: Vec<String> = Vec::new();

        for product in &self.products {
            if product.quantity > 0 {
                products_list.push(String::from( &product.name));

            }
        }


        products_list
    }
    fn get_inventory_value(&self) -> f64 {
        let mut sum = 0.0;
        for product in &self.products {
            sum += product.price;
        }

        sum
    }
}