# Move Semantics

These exercises are adapted from [pnkfelix](https://github.com/pnkfelix)'s [Rust Tutorial](https://pnkfelix.github.io/rust-examples-icfp2014/) -- Thank you Felix!!!

## Further information

For this section, the book links are especially important.

- [Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Reference and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

to_vec() method:

The to_vec() method is specific to vectors (Vec<T>) and creates a new vector with cloned elements from the original vector.

This method creates a deep copy of the vector, meaning that both the vector and its elements are duplicated.
The resulting vector is completely independent of the original vector, and modifying one will not affect the other.
The to_vec() method is useful when you specifically want to create a new vector with cloned elements.

Example: let new_vec = original_vec.to_vec();
clone() method:

The clone() method is a more general method available to all types that implement the Clone trait, including vectors.
It creates a new instance that is a clone of the original object, including all internal data and properties.
For vectors, the clone() method also creates a deep copy, similar to to_vec().

The clone() method can be used to create copies of vectors as well as other types that implement Clone.
Example: let new_vec = original_vec.clone();

In summary, to_vec() is specific to vectors and creates a new vector with cloned elements, while clone() is a more general method that creates a clone of an object, including vectors. They both create deep copies of the original data, but clone() is more versatile as it can be used with any type implementing Clone.
