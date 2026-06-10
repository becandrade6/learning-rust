# Storing Lists of Values with Vectors

The first collection type we'll lok at is `Vec<T>`, also known as a vector. Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

## Creating a New Vector

We added a type annotation when creating a new Vector. Because we aren't inserting any values into this vector, Rust doesn't know what kind of elemends we inted to store. This is an important point. Vectors are implemented using genercs. For now, know that the `Vec<T>` type provided by the standard library can hold any type. So, when we create a vector to hold a specific type, we can specify the type within angle backets.

More often, we'll create a `Vec<T>` with initial values, and Rust will infer the type of value you want to store, so you rarely need to do this type annotation.

Rust conveniently provides the `vec!` macro, which will create a new vector that holds the values you give it.

## Updating a Vector

We can use the `push` method to add elements to a vector.

As with any variable, if we want to be able to change its value, we need to make it mutable using the `mut` keyword.

## Reading Elements of Vectors

There are two ways to reference a value stored in a vector: via indexing or by using the `get` method.

We use the index value of `2` to get the third element because vectors are indexed by number, starting at zero. Using `&` and `[]` gives us a reference to the element at the index value. When we use the `get` method with the index passed as an argument, we get an `Option<&T>` that we can use with `match`.

If we try to use the index way and use an invalid index, the code panics! So, most of the times we use the index way if we really want the program to crash if there's an attempt to access an element past the end of the vector.

If we try to use the `get` method and use an invalid index, it returns None without panicking! We will use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances. Then we need to have logic to handle having either Some or None.

We cannot have mutable and immutable references in the same scope of vectors, such as making a reference to the first element of a vector and later trying to push a value to the end of it. It looks like it should work, but it does not due to the way vectors work: Because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn't enough room to put all the elements next to each other where the vector is currently stored. In thar case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

## Iterating Over the Values in a Vector

To change the value that the mutable reference refers to, we have to use the `*` dereference operator to get to the value in `i` before we can use the `+=` operator.

Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker's rules. If we attempted to insert or remove items in the `for` loop bodies, we would get a compiler error similar to the one we got with the code before. The reference to the vector that the `for` loop holds prevents simultaneous modification of the whole vector.

## Using an Enum to Store Multiple Types

Vectors can only store values that are of the same type. This can be inconvenient; there are definetely use cases for needing to store a list of items of different types. Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!

For example, say we want to get values from a row in a spreadsheet in which some of the columns in the row contain integers, some floating-point numbers, and some strings. We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type: that of the enum. Then, we can create a vector to hold that enum and so, ultimately, hold different types.


Rust needs to know what types will be in the vector at compile time so that it knows exactly how much memory on the heap will be needed to store each element. We must also be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. Using an enum plus a `match` expression means that Rust will ensure at compile time that every possible case is handled.

## Dropping a Vector Drops Its Elements

Like any other `struct`, a vector is freed when it goes out of scope.

When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.


