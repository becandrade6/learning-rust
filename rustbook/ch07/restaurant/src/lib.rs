mod front_of_house;

mod back_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // with the use keyword
    // we bring the crate::front_of_house::hosting module into scope with the 'use' keyword
    // so we only need to specify hosting::add_to_waitlist to call the add_to_waitlist function here
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

mod customer {
    // we need to call the use here because outside it was out of scope and we could not use the
    // add_to_waitlist function inside here
    // we could also rewrite the outside 'use' call using the following:
    // pub use crate::front_of_house::hosting;
    use crate::front_of_house::hosting;

    pub fn customer_eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
