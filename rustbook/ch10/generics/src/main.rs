// the function largest is generic over some type T
// the type T must have the trait PartialOrd implemented
// this function has one parameter named list
// which is a slice of values of type T
// it will return a reference to a value of the same type T
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// defines a Point<T> struct to hold x and y coordinate values of any type
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

// here we implement a method that is only for a 
// Point< f32, f32 > instances
// this means the Point<f32, f32> will have a distance_from_origin method;
// other instances of Point<T, U> where T and U are not f32 will not have this method defined.
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let both_integer = Point {x: 5, y: 10};
    let both_float = Point {x: 1.0, y: 4.0};
    let integer_and_float = Point {x: 5, y: 4.0};

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
