use std::thread;

fn main() {
    println!("Example 1:");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // Making the closure now capture a mutable reference
    println!("Example 2:");
    let mut list2 = vec![1, 2, 3];
    println!("Before defining the closure: {list2:?}");

    let mut borrows_mutably = || list2.push(7);

    borrows_mutably();
    println!("After calling closure: {list2:?}");

    // briefly explore spawning a new thread using a closure that needs the move keyword
    println!("Example 3:");

    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {list3:?}");

    thread::spawn(move || println!("From thread: {list3:?}"))
        .join()
        .unwrap();

}
