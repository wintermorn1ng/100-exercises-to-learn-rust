// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

fn check_product_name (product_name: &String) {
    let len = product_name.len();
    if len == 0 || len > 300 {
        panic!();
    }
}

fn check_quantity_or_unit_price (value: u32) {
    if value <= 0 {
        panic!();
    }
}

impl Order {
    pub fn new (product_name: String, quantity: u32, unit_price: u32) -> Self {
        check_product_name(&product_name);
        check_quantity_or_unit_price(quantity);
        check_quantity_or_unit_price(unit_price);
        Order {
            product_name: product_name,
            quantity: quantity,
            unit_price: unit_price
        }
    }

    pub fn product_name (&self) -> &String {
        &self.product_name
    }

    pub fn set_product_name (&mut self, product_name: String) {
        self.product_name = product_name;
    }

    pub fn quantity (&self) -> &u32 {
        &self.quantity
    }

    pub fn set_quantity (&mut self, quantity: u32) {
        self.quantity = quantity;
    }

    pub fn unit_price (&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_unit_price (&mut self, unit_price: u32) {
        self.unit_price = unit_price;
    }

    pub fn total (&self) -> u32 {
        self.quantity * self.unit_price
    }

}