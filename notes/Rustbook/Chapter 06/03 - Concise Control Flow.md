# Concise Control Flow with `if let` and `let...else`

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.

The syntax if let takes a pattern and an expression separated by an equal sign. It works the same way as a match, where the expression is given to the match and the pattern is its first arm. In this case, the pattern is Some(max), and the max binds to the value inside the Some. We can then use max in the body of the `if let` block in the same way we used max in the corresponding match arm. The code in the `if let` block only runs `if the` value matches the pattern.

Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking match enforces that ensures that you aren’t forgetting to handle any cases. Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else.

