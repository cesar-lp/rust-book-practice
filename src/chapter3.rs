pub fn execute() {
    degrees_converter();
    fibonacci_runner();
    twelve_days_of_christmas();
}

fn degrees_converter() {
    let degrees = 2.5;

    println!(
        "{} Celsius - Fahrenheit {}",
        degrees,
        to_fahrenheit(degrees)
    );
    println!("{} Fahrenheit - Celsius {}", degrees, to_celsius(degrees));
}

fn fibonacci_runner() {
    for value in 1..10 {
        println!("The fibonacci of {} is {}", value, fb(value))
    }
}

fn to_celsius(degrees: f64) -> f64 {
    (degrees - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(degrees: f64) -> f64 {
    (degrees * 9.0 / 5.0) + 32.0
}

fn fb(value: i64) -> i64 {
    if value == 1 {
        1
    } else {
        value * fb(value - 1)
    }
}

pub fn twelve_days_of_christmas() {
    let first_christmas_day_changed = "And a partridge in a pear tree";
    let christmas_days = [
        "A partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "12 drummers drumming",
    ];

    let ordinal_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "11th", "12th",
    ];

    for i in 0..12 {
        println!(
            "\nOn the {} day of Christmas\nMy true love sent to me",
            ordinal_numbers[i]
        );

        for j in (0..i + 1).rev() {
            if i > 0 && j == 0 {
                println!("{}", first_christmas_day_changed);
            } else {
                println!("{}", christmas_days[j])
            }
        }

        if i == 11 {
          println!("{}", first_christmas_day_changed);
        }
    }
}
