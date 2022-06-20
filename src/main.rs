mod chapter1;
mod chapter2;
mod chapter3;
mod chapter4;
mod chapter5;
mod chapter6;
mod chapter7;

// comment the exercises that you don't want to run
fn main() {
    chapter1::execute();
    chapter2::execute();
    chapter3::execute();
    chapter4::execute();
    chapter5::execute();
    chapter6::execute();
    chapter7::eat_at_restaurant();
    chapter7::hosting::add_to_waitlist(); // through re-exporting
}
