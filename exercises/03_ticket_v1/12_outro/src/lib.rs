pub struct Order {

    product_name: String,
    quantity: u64,
    unit_price: u64
}



impl Order {


    fn verify_product_name(product_name: &str) {
            
        
        if product_name.is_empty() {

            panic!("Product_Name cannot be empty");

        }

        if product_name.len() > 300 {

            panic!("product_name is too long");

        }

    }

    fn verify_quantity(quantity: u64) {

        
        if quantity <= 0 {
            panic!("quantity must be greather than zero");
        }
    }

    fn verify_unit_price(unit_price: u64) {

        if unit_price <=0 {

            panic!("unit_price must be greather than zero");
        }

    }


    pub fn new(product_name: String, quantity: u64, unit_price: u64) -> Self{

        Self::verify_product_name(&product_name);
        Self::verify_quantity(quantity);
        Self::verify_unit_price(unit_price);


        Order {
            product_name,
            quantity,
            unit_price
        }

    }

    pub fn total(&self) -> u64 {
        
        self.quantity * self.unit_price
    }


    pub fn product_name(&self) -> &str{
        &self.product_name
    }

    pub fn quantity(&self) -> &u64 {

        &self.quantity
    }

    pub fn unit_price(&self) -> &u64 {

        &self.unit_price
    }


    pub fn set_product_name(&mut self, new_product_name: String) {
        Self::verify_product_name(&new_product_name);
        self.product_name = new_product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u64)  {
        Self::verify_quantity(new_quantity);
        self.quantity = new_quantity;

    }

    pub fn set_unit_price(&mut self, new_unit_price: u64) {
        Self::verify_unit_price(new_unit_price);
        self.unit_price = new_unit_price;

    }

}



