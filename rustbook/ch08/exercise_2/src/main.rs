// Exercise 2: Convert strings to Pig Latin.
// Words starting with a consonant: move it to the end and add "ay" (first -> irst-fay).
// Words starting with a vowel: keep the word and add "hay" (apple -> apple-hay).

fn convert_word(word: &str) -> String {
    // Strings can't be indexed by position in Rust, so use the chars iterator
    // to get the first character. None means the word is empty.
    let first_char = word.chars().next();

    match first_char {
        Some(char) => {
            if ['a', 'e', 'i', 'o', 'u'].contains(&char.to_ascii_lowercase()) {
                format!("{word}-hay")
            } else {
                // Skip chars instead of slicing bytes (&word[1..]), which would
                // panic on multibyte UTF-8 characters like 'ñ'.
                let rest: String = word.chars().skip(1).collect();
                format!("{rest}-{char}ay")
            }
        }
        None => String::new(),
    }
}

fn to_pig_latin(text: &str) -> String {
    let mut converted_words: Vec<String> = Vec::new();

    for word in text.split_whitespace() {
        converted_words.push(convert_word(word));
    }

    converted_words.join(" ")
}

fn main() {
    println!("{}", to_pig_latin("first apple"));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rule 1: first consonant moves to the end, followed by "ay"
    #[test]
    fn consonant_word() {
        assert_eq!(to_pig_latin("first"), "irst-fay");
    }

    // Rule 2: vowel-initial words stay intact and get "hay"
    #[test]
    fn vowel_word() {
        assert_eq!(to_pig_latin("apple"), "apple-hay");
    }

    #[test]
    fn all_five_vowels_are_recognized() {
        assert_eq!(
            to_pig_latin("apple ear igloo oak umbrella"),
            "apple-hay ear-hay igloo-hay oak-hay umbrella-hay"
        );
    }

    // Multi-word input: each word is converted independently
    #[test]
    fn multiple_words() {
        assert_eq!(to_pig_latin("first apple"), "irst-fay apple-hay");
    }

    // Case: uppercase letters follow the same rules, no case change
    #[test]
    fn uppercase_vowel() {
        assert_eq!(to_pig_latin("Apple"), "Apple-hay");
    }

    #[test]
    fn uppercase_consonant() {
        assert_eq!(to_pig_latin("First"), "irst-Fay");
    }

    // UTF-8: multibyte chars must be handled as chars, not bytes
    #[test]
    fn multibyte_first_consonant() {
        assert_eq!(to_pig_latin("ñoque"), "oque-ñay");
    }

    #[test]
    fn multibyte_char_in_middle_of_word() {
        assert_eq!(to_pig_latin("café"), "afé-cay");
    }

    // Edge cases
    #[test]
    fn single_vowel_word() {
        assert_eq!(to_pig_latin("a"), "a-hay");
    }

    #[test]
    fn single_consonant_word() {
        assert_eq!(to_pig_latin("b"), "-bay");
    }

    #[test]
    fn empty_string() {
        assert_eq!(to_pig_latin(""), "");
    }
}
