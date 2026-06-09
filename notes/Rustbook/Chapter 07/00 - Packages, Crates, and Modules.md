# Packages, Crates, and Modules

As you write large programs, organizing your code will become increasingly important. By grouping related functionality and separating code with distinct features, you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.

A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies.

We’ll also discuss encapsulating implementation details, which lets you reuse code at a higher level: Once you’ve implemented an operation, other code can call your code via its public interface without having to know how the implementation works. The way you write code defines which parts are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head.

- Packages: A Cargo feature that lets you build, test and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope and privacy of paths
- Paths: A way of naming an item, such as a struct, function or module
