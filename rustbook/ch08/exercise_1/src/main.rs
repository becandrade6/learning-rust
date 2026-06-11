use std::collections::HashMap;

// Exercise 1:
// Given a list of integers, use a vector and
// return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn main() {
    let numbers = [1, 2, 2, 3, 4];

    match median(&numbers) {
        Some(value) => println!("median: {value}"),
        None => println!("median: empty list"),
    }

    match mode(&numbers) {
        Some(value) => println!("mode: {value}"),
        None => println!("mode: empty list"),
    }
}

fn median(numbers: &[i32]) -> Option<f64> {
    // check if the input list is empty, if yes return None
    if numbers.is_empty() {
        return None;
    }

    // make a vector from the list
    let mut vector = numbers.to_vec();
    // sort the vector
    vector.sort_unstable();

    // get vector length and calculate the mid index
    let length = vector.len();
    let mid = length / 2;

    // check if the vector is even(par) or odd(ímpar)
    if length % 2 == 0 {
        // even length: average of the two middle values
        Some((vector[mid - 1] + vector[mid]) as f64 / 2.0)
    } else {
        Some(vector[mid] as f64)
    }
}

fn mode(numbers: &[i32]) -> Option<i32> {
    // create an empty mutable hash map
    let mut counts = HashMap::new();

    // count the items in the list, making the hash map be {value: count}
    for &number in numbers {
        let count = counts.entry(number).or_insert(0);
        *count += 1;
    }

    // then, check in the hash map which key has the biggest count value
    let mut max_count = 0;
    let mut mode = None;
    for (&number, &count) in &counts {
        if count > max_count {
            max_count = count;
            mode = Some(number);
        }
    }
    mode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_odd_length() {
        assert_eq!(median(&[3, 1, 2]), Some(2.0));
    }

    #[test]
    fn median_even_length_averages_middle_values() {
        assert_eq!(median(&[1, 2, 3, 4]), Some(2.5));
    }

    #[test]
    fn median_even_length_with_same_middle_values() {
        assert_eq!(median(&[1, 3, 3, 5]), Some(3.0));
    }

    #[test]
    fn median_single_element() {
        assert_eq!(median(&[7]), Some(7.0));
    }

    #[test]
    fn median_two_elements() {
        assert_eq!(median(&[1, 2]), Some(1.5));
    }

    #[test]
    fn median_already_sorted() {
        assert_eq!(median(&[1, 2, 3, 4, 5]), Some(3.0));
    }

    #[test]
    fn median_unsorted() {
        assert_eq!(median(&[5, 1, 4, 2, 3]), Some(3.0));
    }

    #[test]
    fn median_negative_numbers() {
        assert_eq!(median(&[-5, -1, -3]), Some(-3.0));
    }

    #[test]
    fn median_empty_returns_none() {
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn median_does_not_mutate_input() {
        let numbers = [3, 1, 2];
        median(&numbers);
        assert_eq!(numbers, [3, 1, 2]);
    }

    #[test]
    fn mode_single_winner() {
        assert_eq!(mode(&[1, 2, 2, 3]), Some(2));
    }

    #[test]
    fn mode_all_same() {
        assert_eq!(mode(&[4, 4, 4]), Some(4));
    }

    #[test]
    fn mode_single_element() {
        assert_eq!(mode(&[9]), Some(9));
    }

    #[test]
    fn mode_most_frequent_at_end() {
        assert_eq!(mode(&[1, 2, 3, 3, 3]), Some(3));
    }

    #[test]
    fn mode_negative_numbers() {
        assert_eq!(mode(&[-1, -1, 2]), Some(-1));
    }

    #[test]
    fn mode_empty_returns_none() {
        assert_eq!(mode(&[]), None);
    }

    // HashMap iteration order is random, so with a tie either
    // value is a valid answer — assert it's one of them.
    #[test]
    fn mode_tie_returns_one_of_the_tied_values() {
        let result = mode(&[1, 1, 2, 2]);
        assert!(result == Some(1) || result == Some(2));
    }
}
