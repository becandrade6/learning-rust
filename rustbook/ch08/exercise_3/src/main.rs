// Exercise 3
// Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company; for example, "Add Sally to Engineering" or "Add Amir to Sales."
// Then, let the user retrieve a list of all people in a department or all people in the company
// by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

// The whole company is just one hash map:
//   key   = department name
//   value = vector with the names of its employees, in insertion order
// (sorting only happens when we list, so adding stays cheap and simple)

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Commands: 'Add <name> to <department>', 'List <department>', 'List All', 'Quit'");

    loop {
        // Reading a line works exactly like the guessing game in chapter 2.
        let mut input = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        // read_line returns how many bytes it read; 0 means the input
        // ended (the user pressed Ctrl+D), so there is nothing left to do.
        if bytes_read == 0 {
            break;
        }

        let line = input.trim();

        if line == "Quit" {
            break;
        }

        let output = run_command(&mut company, line);
        println!("{}", output);
    }
}

/// Puts an employee into a department, creating the department the first
/// time it is mentioned. The entry/or_insert pattern is from chapter 8.3.
fn add_employee(company: &mut HashMap<String, Vec<String>>, name: &str, department: &str) {
    let employees = company.entry(String::from(department)).or_insert(Vec::new());
    employees.push(String::from(name));
}

/// Glues names together like "Amir, Carol, Sally".
/// Built with push_str in a loop, the chapter 8.2 way of growing a String.
fn join_names(names: &Vec<String>) -> String {
    let mut result = String::new();
    for name in names {
        // Put the separator BEFORE every name except the first one.
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(name);
    }
    result
}

/// One line describing a department, e.g. "Engineering: Amir, Sally",
/// with the names sorted alphabetically.
fn list_department(company: &HashMap<String, Vec<String>>, department: &str) -> String {
    // get() returns Option<&Vec<String>>: Some if the department exists,
    // None if it does not. match forces us to handle both cases (ch6).
    match company.get(department) {
        Some(employees) => {
            // Sort a CLONE, not the stored vector itself. Reading the
            // company should not change it, and this also lets the
            // function borrow the map immutably (&) instead of mutably.
            let mut sorted = employees.clone();
            sorted.sort();
            format!("{}: {}", department, join_names(&sorted))
        }
        None => format!("No department named '{}'.", department),
    }
}

/// The whole company, one line per department, departments sorted
/// alphabetically and the names inside each one sorted too.
fn list_all(company: &HashMap<String, Vec<String>>) -> String {
    if company.is_empty() {
        return String::from("The company has no employees yet.");
    }

    // A HashMap stores its keys in arbitrary order (chapter 8.3 warns
    // about this), so we copy the department names into a vector and
    // sort that vector ourselves.
    let mut departments: Vec<String> = Vec::new();
    for department in company.keys() {
        departments.push(department.clone());
    }
    departments.sort();

    // Now build the output by reusing list_department for each line.
    let mut result = String::new();
    for department in &departments {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&list_department(company, department));
    }
    result
}

/// Takes one line the user typed, figures out which command it is, runs it,
/// and returns the text to print. Returning the text (instead of printing
/// here) is what makes this function easy to test.
fn run_command(company: &mut HashMap<String, Vec<String>>, line: &str) -> String {
    // split_whitespace (seen in chapter 8.3) splits the line into words and
    // ignores any extra spaces. We collect the words into a vector so we
    // can count them and look at them by index.
    let mut words: Vec<&str> = Vec::new();
    for word in line.split_whitespace() {
        words.push(word);
    }

    // "Add <name> to <department>" is exactly 4 words shaped Add/?/to/?.
    if words.len() == 4 && words[0] == "Add" && words[2] == "to" {
        let name = words[1];
        let department = words[3];
        add_employee(company, name, department);
        return format!("Added {} to {}.", name, department);
    }

    // "List All" must be checked BEFORE "List <department>", otherwise
    // it would be treated as a department called "All".
    if words.len() == 2 && words[0] == "List" && words[1] == "All" {
        return list_all(company);
    }

    if words.len() == 2 && words[0] == "List" {
        return list_department(company, words[1]);
    }

    // Anything else: tell the user what we do understand.
    format!(
        "Unrecognized command: '{}'. Try 'Add <name> to <department>', \
         'List <department>', 'List All' or 'Quit'.",
        line.trim()
    )
}

// Tests are explained properly in chapter 11. The short version:
// - `cargo test` compiles and runs every function marked #[test]
// - #[cfg(test)] means "only compile this module for cargo test"
// - `use super::*` brings the functions above into the module (ch7)
// - assert_eq!(a, b) makes the test fail if a and b are not equal
#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // Parsing: does run_command understand each command shape?
    // ------------------------------------------------------------------

    #[test]
    fn add_stores_the_employee_and_confirms() {
        let mut company = HashMap::new();

        let output = run_command(&mut company, "Add Sally to Engineering");

        assert_eq!(output, "Added Sally to Engineering.");
        // Also check it really landed in the map, not just the message.
        assert_eq!(company.get("Engineering"), Some(&vec![String::from("Sally")]));
    }

    #[test]
    fn extra_whitespace_is_ignored() {
        let mut company = HashMap::new();

        let output = run_command(&mut company, "  Add   Amir   to   Sales  ");

        assert_eq!(output, "Added Amir to Sales.");
    }

    #[test]
    fn list_all_is_not_a_department_called_all() {
        let mut company = HashMap::new();
        run_command(&mut company, "Add Sally to Engineering");

        let output = run_command(&mut company, "List All");

        // If "List All" were parsed as List <department>, the answer
        // would be "No department named 'All'." instead.
        assert_eq!(output, "Engineering: Sally");
    }

    #[test]
    fn empty_input_is_rejected() {
        let mut company = HashMap::new();

        let output = run_command(&mut company, "   ");

        assert!(output.contains("Unrecognized"));
    }

    #[test]
    fn unknown_command_is_rejected() {
        let mut company = HashMap::new();

        let output = run_command(&mut company, "Remove Sally from Engineering");

        assert!(output.contains("Unrecognized"));
    }

    #[test]
    fn malformed_add_is_rejected() {
        let mut company = HashMap::new();

        assert!(run_command(&mut company, "Add Sally").contains("Unrecognized"));
        assert!(run_command(&mut company, "Add Sally to").contains("Unrecognized"));
        assert!(run_command(&mut company, "Add to Engineering").contains("Unrecognized"));
        // None of those should have created anything.
        assert!(company.is_empty());
    }

    #[test]
    fn list_with_no_department_is_rejected() {
        let mut company = HashMap::new();

        let output = run_command(&mut company, "List");

        assert!(output.contains("Unrecognized"));
    }

    // ------------------------------------------------------------------
    // The hash map + vectors logic
    // ------------------------------------------------------------------

    #[test]
    fn list_department_is_sorted_alphabetically() {
        let mut company = HashMap::new();
        // Inserted in the WRONG order on purpose: the test only proves
        // something if the unsorted answer would be different.
        add_employee(&mut company, "Sally", "Engineering");
        add_employee(&mut company, "Amir", "Engineering");
        add_employee(&mut company, "Carol", "Engineering");

        let output = list_department(&company, "Engineering");

        assert_eq!(output, "Engineering: Amir, Carol, Sally");
    }

    #[test]
    fn listing_does_not_modify_the_company() {
        let mut company = HashMap::new();
        add_employee(&mut company, "Sally", "Engineering");
        add_employee(&mut company, "Amir", "Engineering");

        // Asking twice must give the same answer: listing sorts a clone,
        // it must not rearrange or drain the stored vector.
        let first = list_department(&company, "Engineering");
        let second = list_department(&company, "Engineering");

        assert_eq!(first, second);
    }

    #[test]
    fn unknown_department_gives_a_message_not_a_panic() {
        let company = HashMap::new();

        let output = list_department(&company, "Marketing");

        assert_eq!(output, "No department named 'Marketing'.");
    }

    #[test]
    fn departments_are_independent() {
        let mut company = HashMap::new();
        add_employee(&mut company, "Sally", "Engineering");
        add_employee(&mut company, "Amir", "Sales");

        assert_eq!(list_department(&company, "Engineering"), "Engineering: Sally");
        assert_eq!(list_department(&company, "Sales"), "Sales: Amir");
    }

    #[test]
    fn duplicate_names_are_kept() {
        // Two employees can share a name; both must appear.
        let mut company = HashMap::new();
        add_employee(&mut company, "Amir", "Sales");
        add_employee(&mut company, "Amir", "Sales");

        assert_eq!(list_department(&company, "Sales"), "Sales: Amir, Amir");
    }

    #[test]
    fn list_all_sorts_departments_and_names() {
        let mut company = HashMap::new();
        // Departments AND names inserted out of order on purpose.
        add_employee(&mut company, "Sally", "Sales");
        add_employee(&mut company, "Zoe", "Engineering");
        add_employee(&mut company, "Amir", "Engineering");

        let output = list_all(&company);

        assert_eq!(output, "Engineering: Amir, Zoe\nSales: Sally");
    }

    #[test]
    fn list_all_on_empty_company() {
        let company = HashMap::new();

        let output = list_all(&company);

        assert_eq!(output, "The company has no employees yet.");
    }

    #[test]
    fn join_names_puts_commas_between_names() {
        assert_eq!(join_names(&vec![]), "");
        assert_eq!(join_names(&vec![String::from("Amir")]), "Amir");
        assert_eq!(
            join_names(&vec![String::from("Amir"), String::from("Sally")]),
            "Amir, Sally"
        );
    }
}
