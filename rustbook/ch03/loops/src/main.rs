fn main() {
    let mut count = 0;

    // give a label to the loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // break the labeled loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    println!("-----------------");

    // While loop example
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    println!("-----------------");
    // looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // the old way with while
    while index < 5 {
        println!("the value with while is {}", a[index]);
        index += 1;
    }

    // now using the for loop
    for element in a {
        println!("the value with for is {element}");
    }

    println!("-----------------");
    // now countdown using a for loop and rev to reverse a range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
