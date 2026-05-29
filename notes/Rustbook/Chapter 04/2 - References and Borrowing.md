# References
A reference is guaranteed to point to a valid value of a particular type for the life of that reference.

Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. We’ll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.

We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

So, what happens if we try to modify something we’re borrowing? Spoiler alert: it doesn't work!

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

# Mutable References

We can fix the code mentioned before, to allow us to modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:

First, we change s to be mut. Then, we create a mutable reference with &mut s where we call the change function and update the function signature to accept a mutable reference with some_string: &mut String. This makes it very clear that the change function will mutate the value it borrows.

Mutable references have one big restriction: **if you have a mutable reference to a value, you can have no other references to that value.**

Rust enforces a similar rule for combining mutable and immutable references. We cannot borrow a variable as immutable and mutable.

We *also* cannot have a mutable reference while we have an immutable one to the same value.

Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

# Dangling References

In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

# The Rules of References

- At any given time, you can have *either* one mutable reference *or* any number of immutable references.
- References must always be valid.
