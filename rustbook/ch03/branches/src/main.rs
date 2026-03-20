fn main() {
    let number = 3;

    // The condition here must be a bool
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // we cant just use if number, it would give an error
    // so, if we want to check if the number is not zero
    if number != 0 {
        println!("number was something other than zero");
    }

    // Handling multiple conditions by combining if and else in an else if
    let number2 = 6;

    if number2 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // as if is an expression, we can use it on
    // the right side of a let statement to assign the outcome to a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("the value of number is {number}");

    
}
