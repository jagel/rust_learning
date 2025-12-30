fn main() {
    println!("==== String and loop example ===");

    let mut str1 = String::from("Hello, 😊 world!");

    println!("Original string: {str1}");

    str1.push_str("A");
    println!("After push_str: {str1}");

    for byte in str1.bytes() {
        print!("{} ", byte);
    }

    for chr in str1.chars() {
        print!("{chr} ");
    }

    for (pos, chr) in str1.char_indices() {
        println!("Position: {pos}, Character: {chr}");
    }

    let result = function_1(2);
    let final_result = function_2(result);
    loop_examples();
}

fn function_1 (input: i32) -> i32 {
    println!("input = {input}");
    input + 1
    // returns input + 1;
}
fn function_2 (num: i32) -> i32 {
    let result = if num % 2 == 0 {
        println!("{num} is even");
        num + 10
    } else {
        println!("{num} is odd");
        num + 1
    };
    result
}
//example of loops
fn loop_examples() {
    let mut count = 0;

    // Infinite loop with break
    loop {
        count += 1;
        if count == 5 {
            println!("Breaking the loop at count = {count}");
            break;
        }
    }

    // While loop
    while count < 10 {
        println!("Count in while loop: {count}");
        count += 1;
    }

    // nested loops
    'mainloop: while count < 15 {
        'innerloop: while count < 20 {
            count += 1;
            if count == 17 {
                break 'mainloop; // Breaks the outer loop
            }
        }
        println!("Count in while loop: {count}");
        count += 1;
    }

    // For loop over a range
    for i in 0..5 {
        println!("For loop iteration: {i}");
    }

    // For loop over an array
    let arr = [10, 20, 30, 40, 50];
    for &item in arr.iter() {
        println!("Array item: {item}");
    }
}