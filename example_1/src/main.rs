use std::io;
use std::mem; // for memory size functions

 fn main() {
    println!("Please enter a number:");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    
    let item = false;
    println!("The value of item is: {}", item);

    println!("You entered: {input}");

    //creating unit type
    let unit: () = (); // this is used to represent an empty value or no value at all
    // Creating tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("The value of the tuple is: {tup:?}"); // Debug print

    // Accessing tuple elements
    let (x, y, z) = tup; // Destructuring
    println!("The value of y is: {y}");

    // Creating array
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // Save in stack

    // Creating an array with same values
    let arr2: [i32; 5] = [3; 5]; // [3, 3, 3, 3, 3]

    // Create an slice
    let slice: &[i32] = &arr2[1..4]; // Slice from index 1 to 3

    println!("The slice is: {}", arr2.len());
    println!("Memory size of arr2: {} bytes", mem::size_of_val(&arr2));

    let slice_arr: &[i32] = &arr[1..4]; // Slice of entire array
    println!("Slice of arr : {:?}", slice_arr); // index range 1 to 3
    println!("Slice of arr : {:?}", &arr[1..=4]); // index range 1 to 4
    println!("Slice of arr : {:?}", &arr[1..5]); // index range 1 to 4
}