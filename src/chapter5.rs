struct User {
    first_name: String,
    last_name: String,
    email: String,
    age: u8,
}

// tuple struct
struct Color(i32, i32, i32);

// unit like struct
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn execute() {
    let _user = build_user(String::from("Jhon"), String::from("Doe"));
    create_user_with_destructuring(_user);
    build_color_tuple_struct();
    create_unit_like_struct();

    // keep ownership of rectangle
    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    println!(
        "The area of the rectangle {:?} is {}",
        rectangle,
        rectangle.area()
    );

    can_hold();

    println!("Creating square {:?}", Rectangle::square(10));
}

fn build_user(first_name: String, last_name: String) -> User {
    User {
        first_name,
        last_name,
        email: String::from("jhon.doe@email.com"),
        age: 18,
    }
}

fn create_user_with_destructuring(user: User) -> User {
    User { age: 20, ..user }
    // value moved, cannot use
    // println!("{}", user.first_name)
}

fn build_color_tuple_struct() -> Color {
    Color(255, 255, 255)
}

fn create_unit_like_struct() {
    let _condition = AlwaysEqual;
}

fn can_hold() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "Can {:?} hold {:?}? {}",
        rect1,
        rect2,
        rect1.can_hold(&rect2)
    );
    println!(
        "Can {:?} hold {:?}? {}",
        rect1,
        rect3,
        rect1.can_hold(&rect3)
    );
}
