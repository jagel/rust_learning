
// const variables computed at compile time
const MAX_POINTS: u32 = 100_000;


fn main() {
    let s: &str = "Hello, world!";
    print!("{} \n {}",s, s);
    println!("{}", s);

    // Error messages
    eprint!("{}\n", s);
    eprintln!("{}", s);

    let name : &str = "Javier";
    let format_name: String = format!("Name: {}", name);
    println!("Name: {name}");

    println!("{}", MAX_POINTS);


    let x: i32 = 5;
    println!("The value of x is: {}", x);

    // x = 6; // This will cause a compile-time error because x is immutable
    
    let mut y: i32 = 10;
    println!("The initial value of y is: {}", y);
    y = 15; // This is allowed because y is mutable
    println!("The updated value of y is: {}", y);

    // List all the primitive types in Rust
    let a: i8 = -128; // 8-bit signed integer [-128 to 127]
    let b: u8 = 255;  // 8-bit unsigned integer [0 to 255]
    let c: i16 = -32768; // 16-bit signed integer [-32768 to 32767]
    let d: u16 = 65535;  // 16-bit unsigned integer [0 to 65535]
    let e: i32 = -2147483648; // 32-bit signed integer [-2147483648 to 2147483647]
    let f: u32 = 4294967295;  // 32-bit unsigned integer [0 to 4294967295]
    let g: i64 = -9223372036854775808; // 64-bit signed integer [-9223372036854775808 to 9223372036854775807]
    let h: u64 = 18446744073709551615;  // 64-bit unsigned integer
    let i: i128 = -170141183460469231731687303715884105728; // 128-bit signed integer
    let j: u128 = 340282366920938463463374607431768211455;  // 128-bit unsigned integer
    let jj: i128 = i128::MIN; // Minimum value for 128-bit signed integer
    let jjj: u128 = u128::MAX; // Maximum value for 128-bit unsigned integer
    let k: f32 = 3.14; // 32-bit floating point
    let l: f64 = 2.718281828459045; // 64-bit floating point
    let m: bool = true; // Boolean
    let n: char = 'R'; // Character

    // Variable shadowing
    let p: i32 = 10;
    let p: i32 = p + 5; // Shadows previous p
    println!("The value of p after shadowing is: {}", p);
    // Shadowing in Rust allows you to reuse the same variable name in the same scope, 
    //creating a new variable that shadows the previous one.

    // inner scope example
    let q: i32 = 20;
    {
        let q: i32 = q * 2; // Shadows outer q
        println!("The value of q in the inner scope is: {}", q);
    }
    println!("The value of q in the outer scope is: {}", q);
}
