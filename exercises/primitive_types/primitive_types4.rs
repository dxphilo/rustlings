// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}

// In this code, &a[1..4] creates a reference to the array slice that starts at index 1 and ends at index 3 (both inclusive). The resulting slice will contain the elements [2, 3, 4] copied from the original array.

// Remember that the indexing in Rust starts from 0, so index 1 corresponds to the second element of the array, which is 2. The range 1..4 is exclusive on the upper bound, so it will include elements up to index 3 (4 in the original array).