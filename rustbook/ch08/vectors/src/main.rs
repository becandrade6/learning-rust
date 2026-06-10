use std::vec;

fn main() {
    // Creating a new vector
    // This creates a new, empty vector
    let v: Vec<i32> = Vec::new();

    // This creates a new vector that holds the values we give
    let v2 = vec![1, 2, 3];

    // Create a new empty mutable vector
    // as we are pushing values later, there is no need for type annotation, Rust infers it
    let mut mutable_vector = Vec::new();

    // Add elements to the mutable vector
    mutable_vector.push(5);
    mutable_vector.push(6);
    mutable_vector.push(7);
    mutable_vector.push(8);

    //---------------- Reading Elements of Vectors ---------------------------
    let v3 = vec![1, 2, 3, 4, 5];
    // Read the element via indexing
    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    // Read the element via get
    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // the code bellow if uncommented will not compile
    // because we can't have mutable and immutable references in the same scope
    // this occurs because on how vectors work
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];                           // immutable borrow here
    // v.push(6);                                   // mutable borrow here
    // println!("The first element is {first}")     // immutable borrow later used here

    //---------------- Iterating Over the Values in a Vector ---------------------------
    // how to use a for loop to get immutable references to each element in a vector of i32 values and print them
    let v = vec![100, 32, 57];
    for i in v {
        println!("For loop with immutable references: {i}");
    }

    // how to iterate over mutable references to each element in a mutable vector in order
    // to make changes to all the elements. Lets add 50 to each element
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // add 50 to each element
        // needs to use the * dereference operator to get to the value in i, before using the += operator
        *i += 50;
        println!("For loop with mutable references after adding 50: {i}");
    }

    //---------------- Using an Enum to Store Multiple Types ---------------------------
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here, all of its contents are also dropped

}
