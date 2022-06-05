pub fn execute() {
    invalid_reference();
    clone();
    ownership();
    mut_inmut_references();
    slices();
}

/*
  In other languages this could cause a double free error.
  Rust invalidates the first reference, this is known as a "move"
  (s1 was moved into s2).
  This also implies that Rust will never automatically create deep copies
*/
fn invalid_reference() {
    let s1 = String::from("Hello!");
    // let s2 = s1;

    println!("{}", s1);
}

// Heap data is copied
fn clone() {
    let s1 = String::from("Hello!");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownership() {
    let s = String::from("hello");

    does_not_take_ownership(&s);
    let s = takes_and_returns(s);
    takes_ownership(s);

    // this throws an error because "s" is moved
    // println!("{}", s);
}

fn does_not_take_ownership(value: &String) {
    println!("String: {}", value);
}

fn takes_and_returns(value: String) -> String {
    value
}

fn takes_ownership(value: String) {
    println!("String: {}", value);
}

fn mut_inmut_references() {
    let mut s = String::from("Helo!");

    let r1 = &s;
    let r2 = &s;

    // error, s is borrowed as inmutable,
    // therefore we cannot borrow it as mutable
    // let r3 = &mut s;

    // r1 and r2 will not be used after this println
    println!("{} and {}", r1, r2);

    // because r1 and r2 are not used after the println statement before (scopes not overlaped),
    // we can ensure that the mutable reference will not cause any conflict
    let r3 = &mut s;

    println!("{}", r3);
}

pub fn slices() {
    let hello_world = String::from("Hello world");
    let _hello = &hello_world[..5];
    let _world = &hello_world[6..];

    first_word(&hello_world);
    first_word("Hello world");
}

// deref coercion
fn first_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[..i];
        }
    }

    &sentence[..]
}
