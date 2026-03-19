## Boas praticas

Nomes dos arquivos de código Rust sempre minúsculo e com snake case caso seja mais de uma palavra, por exemplo se fossemos criar um arquivo para um hello world poderíamos criar o arquivo `hello_world.rs` e **não** o arquivo `helloworld.rs`.

Todo bloco de função tem que estar entre chaves ({}) e a chave que abre a função, **deve** estar ao lado da função, por exemplo:

```rust
fn main() {
	// codigo aqui
}
```


## Hello world

A função `main`é especial: ela é sempre o primeiro código que roda em todo e qualquer programa Rust executável.

using a `!` means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions

Most lines of Rust code end with a semicolon (`;`).

Before running a Rust program, you must compile it using the Rust compiler by entering the `rustc` command and passing it the name of your source file, like this:

`$ rustc main.rs`

Then, Rust outputs a binary executable.

## Hello, Cargo!

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs _dependencies_.)

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the _target/debug_ directory.

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in _target/release_ instead of _target/debug_. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in _target/release_.

