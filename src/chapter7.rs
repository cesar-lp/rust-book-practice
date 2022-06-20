use foods::{Appetizer, Breakfast};
pub use front_of_house::hosting;

mod back_of_house;
mod foods;
mod front_of_house;

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // absolute path if this was a lib/binary file
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Won't compile, prop is not public
    // meal.seasonal_fruit = String::from("blueberries");

    let saladAppetizer = Appetizer::Salad;
}
