use std::f32::consts::E;
use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {

    // The return type of File::open is a Result<T, E>
    // the T in the implementation of open has been filled with the type of
    // the success value std::fs::File , which is a file handle.
    // the type of E used in the error value is std::io::Error
    // this return type means the call might succeed and return a file handle we can red from or write to
    // the function call also might fail, so the Error tell us this
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        }
    };

    // we can write this as well... the expect lets us choose the panic error message
    // it will return the value inside the Ok if Result is Ok. Otherwise will panic with the message
    let greeting_file = File::open("hello2.txt")
        .expect("hello2.txt should be included in this project");


}

// example of error propagation
// instead of handling the error within the function, we
// return the error so the calling code can decide what to do
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file)=> file,
        Err(e)=> return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
