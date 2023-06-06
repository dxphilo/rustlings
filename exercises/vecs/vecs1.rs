// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.


use std::vec;

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = a.to_vec();

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}


// To assign a plain array a to a vector in Rust, you can use the to_vec method. Here's an example:

// rust
// Copy code
// let a = [10, 20, 30, 40];
// let vec_a: Vec<_> = a.to_vec();

// println!("{:?}", vec_a);
// In the above code, the to_vec method is called on the array a, 
// which converts it into a Vec<_> (a vector with the same element type as the array). 
// The resulting vector is assigned to the variable vec_a.

// The println! macro is then used to print the contents of the vec_a vector.

// Note that to_vec creates a new vector and copies the elements of the array into it. 
// If you modify the vector vec_a, it won't affect the original array a because they are separate data structures.