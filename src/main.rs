mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}
        fn _serve_order() {}
        fn _take_payment() {}

        fn _fix_incorrect_order() {
            _cook_order();
            _serve_order();
        }

        fn _cook_order() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // won't compile

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::collections::HashMap;

use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", random_number);

    let mut map = HashMap::new();
    map.insert(1, 2);

    eat_at_restaurant();
}
