// This is how we define a User struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Defining tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Defining a Unit-Like struct
struct AlwaysEqual;
fn main() {

    // Creating an instance of the User struct
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // We use dot notation to get/change a specific value from a struct
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1     // struct update syntax
    };

    // black and origin values are different types because they are instances
    // of different tuple structs. each struct you define is its own type
    // A function that takes a parameter of type Color cannot take a Point as an argument
    let black = Color(0, 0, 0);
    
    let origin = Point(0, 0, 0);

    // Destructuring a tuple struct requires the name of the type before
    let Point(x, y, z) = origin;


}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,   // field init shorthand
        email,      // field init shorthand
        sign_in_count: 1,
    }
}
