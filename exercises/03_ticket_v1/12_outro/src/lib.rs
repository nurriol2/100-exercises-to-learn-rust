//  TODO:
//  Define a new `Order` type.
//  It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//  The product name can't be empty
//  The product name can't be longer than 300 bytes.
//  The quantity must be strictly greater than zero.
//  Use a sufficiently sized data type for unit_price <= The unit price is in cents
//  The unit price must be strictly greater than zero.
//  Order must include a method named `total`
//  `total` returns the total price of the order.
//  Order must provide setters for each field.
//  Order must provide getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: usize,
    unit_price: usize,
}

impl Order {
    pub fn new(product_name: String, quantity: usize, unit_price: usize) -> Order {
        validate_non_empty(&product_name, "Product name".into());
        validate_max_len(&product_name, 300, "Product name".into());
        validate_non_zero(quantity, "Quantity".into());
        validate_non_zero(unit_price, "Unit Price".into());

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> usize {
        self.quantity * self.unit_price
    }

    // Setters
    pub fn set_product_name(&mut self, product_name: String) {
        validate_non_empty(&product_name, "Product name".into());
        validate_max_len(&product_name, 300, "Product name".into());
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: usize) {
        validate_non_zero(quantity, "Quantity".into());
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: usize) {
        validate_non_zero(unit_price, "Unit Price".into());
        self.unit_price = unit_price;
    }

    // Getters
    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &usize {
        &self.quantity
    }

    pub fn unit_price(&self) -> &usize {
        &self.unit_price
    }
}

// Validation
// WARN: Panics on failure
fn validate_non_empty(arg: &String, arg_name: String) {
    if arg.is_empty() {
        panic!("{} cannot be empty.", arg_name);
    }
}

// WARN: Panics on failure
fn validate_max_len(arg: &String, max: u16, arg_name: String) {
    if arg.len() as u16 > max {
        panic!("{} cannot be longer than {} bytes.", arg_name, max);
    }
}

// WARN: Panics on failure
fn validate_non_zero(arg: usize, arg_name: String) {
    if arg <= 0 {
        panic!("{} must be strictly greater than zero.", arg_name);
    }
}
