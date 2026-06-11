fn main() {
    use std::collections::HashMap;
    // --------- Creating a New Hash Map ------------------
    // one way to create an empty hash map is to use new
    // and add elements with insert
    // here we are keeping track of the scores of two teams
    // whose names are Blue and Yellow
    // the Blue team starts with 10 points and Yellow with 50
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // --------- Accessing values in a hash map ------------------
    // Here, score will have the value thats associated with the Blue team, and the result will be 10.
    // The get method returns an Option<&V> if there's no value for that key in the hash map
    // get will return None. This program handles the Option by calling copied to get an Option<i32>
    // rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // we can iterate over each key-value pair in a hash map in a similar manner as we do with vectors
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // --------- Ownership in Hash Maps ------------------
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point! The values were moved into the hash map
    // and the hash map is the owner now of those values
    // try using them and see what compiler error you get. ("borrow of moved value")
    //println!("{field_name}: {field_value}");

    // --------- Updating a Hash Map ------------------

    // Overwriting a value
    // here we overwrite 10 with 25, the print will show 25
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // Adding a key and value only if a key isn't present

    // the entry method takes the key you want to check (if has a value or not) as a parameter
    // and returns an enum called Entry that represents a value that might or might not exist
    // the 'or_insert' method on Entry is defined to return a mutable reference to the value for the
    // corresponding Entry key if that key exists, and if not, it inserts the parameter as the new
    // value for this key and returns a mutable reference to the new value.
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // this will insert Yellow with 50
    scores.entry(String::from("Blue")).or_insert(50);   // this will not insert 50 in Blue, because Blue already exists

    // Updating a value based on the old value

    // this counts how many times each word appears in the text
    // the split_whitespace method returns an iterator over subslices, separated by whitespace
    // the or_insert method returns a mutable reference (&mut V) to the value for the specified key
    // then we use the count variable to change the that value
    // the mutable reference goes out of scope at the end of the for loop. so all of these changes are safe and allowed

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}
