#[cfg(test)]
mod tests {
    #[test]
    fn eat_at_restaurant_works() {
        super::eat_at_restaurant();
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {

        }

        fn seat_at_table() {

        }
    }

    mod serving {
        fn take_order() {

        }

        fn server_order() {

        }

        fn take_payment() {

        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

use self::back_of_house::Breakfast;
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal: Breakfast = Breakfast::summer("white");
    println!("Original toast choice: {}", meal.toast);
    meal.toast = String::from("wheat");
    println!("New toast choice: {}", meal.toast);

    hosting::add_to_waitlist();
}