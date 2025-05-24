// A front of house module containing other modules
// Group related definitions together, easier to navigate
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        // variants private by default
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        // variants public by default
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    // change mind about bread
    meal.toast = String::from("whole wheat");
    println!("I'd like {} toast plwase", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}
