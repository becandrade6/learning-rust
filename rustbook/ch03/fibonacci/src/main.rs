use std::io;

fn main() {
    loop {
        // prompt the which Nth number of the sequence the user wants
        println!("Welcome to the Fibonacci sequence generator!");
        
        let nth_term: u32 = get_nth_term();

        println!("The Fibonacci sequence until the {nth_term} term is: ");
        print_fibonacci_sequence_to_nth_term(nth_term);
    }
}

fn get_nth_term() -> u32 {
    loop {
        println!("Please input the Nth term you want to compute in the Fibonacci sequence:");
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read value");

        match value.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Failed to read the Nth term. Try again!");
            }
        }
    }   
}

fn print_fibonacci_sequence_to_nth_term(nth_term: u32) {
    // if input is 0, print nothing
    if nth_term == 0 {
        return;
    }

    // create mutable auxiliary variables, initiating the sequence
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    // loop through 0 to the nth_term (excluding the nth_term, to include would be 0..=nth_term)
    for i in 0..nth_term {
        // always print a comma after the number
        if i > 0 {
            print!(", ");
        }
        // print the current term of the sequence
        print!("{}", a);

        // advance: b becomes the new current term, a+b becomes the next
        // update both variables simultaneously
        // (b, a + b) is fully evaluated before anything is assigned
        // so, a gets the old value of b and b gets the old value of a + b
        (a, b) = (b, a + b);
    }
    // print a new line
    println!();
}
