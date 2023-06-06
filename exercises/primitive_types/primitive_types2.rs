// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)


fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }


    let your_character = 'D';
     // Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

// In Rust, both double quotes ("") and single quotes ('') are used to denote strings, but they have different meanings and use cases.

// Double quotes: ""
// Double quotes are used to create a string literal. String literals are mutable, growable, and commonly used to represent sequences of characters. They can contain escape sequences like \n for a new line, \t for a tab, or \" to include a double quote within the string.

// Example:

// rust
// Copy code
// let double_quoted = "This is a string literal.";
// Single quotes: ''
// Single quotes are used to represent individual characters or literals of type char. Characters in Rust are Unicode scalar values, which can be any Unicode character, including ASCII characters and emoji. Unlike strings, characters are immutable and have a fixed size of 4 bytes.

// Example:

// rust
// Copy code
// let single_quoted: char = 'a';
// In summary, double quotes are used for creating string literals, whereas single quotes are used for representing individual characters.