pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast { 
            toast: String::from(toast), 
            seasonal_fruit: String::from("peaches") 
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}

fn fix_incorrect_order() {
    cook_order();
    // the deliver_order is in the parent module, so we use super to go back one "folder" in the tree
    super::deliver_order();
}

fn cook_order() {}
