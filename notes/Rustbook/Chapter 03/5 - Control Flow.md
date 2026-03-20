he ability to run some code depending on whether a condition is `true` and the ability to run some code repeatedly while a condition is `true` are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are `if` expressions and loops.

## `if` Expressions

An if expressions allows you to branch your code depending on conditions. You provide a condition and then state, if it is met, run this block of code. It it is not met, do not run this block of code.

Blocks of code associated with the conditions in `if` expressions are sometimes called _arms_, just like the arms in `match` expressions that we discussed in the [“Comparing the Guess to the Secret Number”](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number) section of Chapter 2.

It is also worth noting that the condition *must* be a bool. If the condition isn't a bool, we will get an error.

We can use the if expression on the right side of let, when declaring a variable.

But, the code blocks that execute after the if or else or else if must all return the same type. Otherwise, we will get an error.

## Loops

Rust has three kinds of loops: `loop`, `while`, and `for`.

The `loop` keyword tells Rust to execute a block of code over and over again either forever or until you explicitly tell it to stop.

You can place the `break` keyword within the loop to tell the program when to stop executing the loop.

We also used `continue` in the guessing game, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go for the next iteration.

One of the uses of a `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the `break` expression you use to stop the loop; that value will be returned out of the loop so that you can use it.

If you have loops within loops, `break`and `continue` apply to the innermost loop at that point. You can optionally specify a *loop label* on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.

A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop. It's possible to implement behavior like this using a combination of loop, if, else and break; However, this pattern is so common that Rust has a built-in language construct for it, called a while loop. 