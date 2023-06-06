// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand for a hint.


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}

// In Rust, tuples are a way to group together multiple values of different types into a single compound value. 
//They are fixed-size and can store elements of different types. 
//Tuples are defined using parentheses () and can be used to hold a finite number of elements.

// Here's an example of how to create and use tuples in Rust:

// rust
// Copy code
// fn main() {
//     // Creating a tuple
//     let my_tuple = (1, "hello", 3.14);

//     // Accessing tuple elements using indexing
//     let first_element = my_tuple.0;
//     let second_element = my_tuple.1;
//     let third_element = my_tuple.2;

//     // Printing tuple elements
//     println!("First element: {}", first_element);
//     println!("Second element: {}", second_element);
//     println!("Third element: {}", third_element);

//     // Destructuring a tuple
//     let (x, y, z) = my_tuple;
//     println!("x: {}, y: {}, z: {}", x, y, z);
// }
// In this example, we create a tuple my_tuple with three elements of different types: an integer, a string, and a floating-point number. 
// We can access individual elements using dot notation followed by the index (starting from 0).
// Alternatively, we can use destructuring to bind the tuple elements to individual variables.

// fn main() {
//     // Creating a tuple
//     let my_tuple = (1, "hello", 3.14);

//     // Accessing tuple elements using indexing
//     let first_element = my_tuple.0;
//     let second_element = my_tuple.1;
//     let third_element = my_tuple.2;

//     // Printing tuple elements
//     println!("First element: {}", first_element);
//     println!("Second element: {}", second_element);
//     println!("Third element: {}", third_element);

//     // Destructuring a tuple
//     let (x, y, z) = my_tuple;
//     println!("x: {}, y: {}, z: {}", x, y, z);
// }


// Tuples are often used when you want to return multiple values from a function, 
// as they can hold different types of data in a single return value. Additionally, 
// tuples can be passed as function arguments or used to store related but different types of data together.
