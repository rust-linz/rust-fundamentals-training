use std::mem;

/**
Slices
Slices are dynamically sized array-like objects. The term dynamically sized means that their size
is not known at compile time. Yet, like arrays, these don’t expand or contract.

The use of the word dynamic in dynamically sized is closer in meaning to dynamic typing
rather than movement. The lack of compile-time knowledge explains the distinction in the
type signature between an array ([T;n]) and a slice ([T]).

Slices are important because it’s easier to implement traits for slices than arrays.
Traits are how Rust programmers add methods to objects. As [T; 1], [T; 2], ..., [T; n]
are all different types, implementing traits for arrays can become unwieldy. Creating
a slice from an array is easy and cheap because it doesn’t need to be tied to any specific size.
Another important use for slices is their ability to act as a view on arrays (and other slices).
The term view here is taken from database technology and means that slices can gain fast,
 read-only access to data without needing to copy anything around. */

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", ys[0]);
    println!("second element of the array: {}", ys[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Out of bound indexing causes compile error
    //println!("{}", xs[5]);
}
