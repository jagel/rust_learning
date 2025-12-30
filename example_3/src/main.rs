#[derive(Debug)]// derive is used to automatically implement the Debug trait for the enum
enum MySelector {
    OptionA,
    OptionB,
    OptionC,
}

#[derive(Debug)]// derive is used to automatically implement the Debug trait for the enum
enum MyOtherSelector {
    Choice1,
    Choice2,
    Choice3,
    DeatachSelector(String), // enum variant with associated data
}

#[derive(Debug)]// derive is used to automatically implement the Debug trait for the enum
enum Category<T> {
    Choice1,
    Choice2,
    Choice3,
    DeatachSelector(T), // enum variant with associated data
}

// Creating methods associated with the enum
impl MySelector{
    fn selector_description(&self) -> String {
        // add a switch case for each enum variant
        let dft = match self {
            MySelector::OptionA => "This is Option A".to_string(),
            MySelector::OptionB => "This is Option B".to_string(),
            MySelector::OptionC => "This is Option C".to_string(),
            _=> "Unknown Option".to_string(),
        };
        dft
    } 
}

impl Category<String>{
    fn category_description(&self) -> String {
        // add a switch case for each enum variant
        match self {
            Category::Choice1 => String::from("This is Choice 1"),
            Category::Choice2 => String::from("This is Choice 2"),
            Category::Choice3 => String::from("This is Choice 3"),
            Category::DeatachSelector(data) => format!("This is Detached with data: {}", data),
            _=> "Unknown Choice".to_string(),
        }
    }

    fn is_detached(&self) -> bool {
        if let Category::Choice1 = self {
            true
        } else {
            false
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    println!("=== Enum Example and matches ===");

    let selected_a = MySelector::OptionA;
    println!("Selected option: {selected_a:?}"); // Using the Debug trait to print the enum variant

    let selected_b = MySelector::OptionB;
    println!("Selected option: {selected_b:?}");

    let selected_c = MySelector::OptionC;
    println!("Selected option: {selected_c:?}");

    let selected_str = selected_a.selector_description();
    println!("Description: {selected_str}");

    let selected_b_global = match selected_b {
        MySelector::OptionA => "You selected Option A",
        MySelector::OptionC | MySelector::OptionB => "You selected Option C or B",
        _ => "Unknown option",
    };
    println!("Global selection: {selected_b_global}");

    let other_selector_1 = MyOtherSelector::Choice1;
    println!("Other selector choice 1: {other_selector_1:?}");
    let other_selector_2 = MyOtherSelector::Choice2;
    println!("Other selector choice 2: {other_selector_2:?}");
    let other_selector_3 = MyOtherSelector::Choice3;
    println!("Other selector choice 3: {other_selector_3:?}");

    let other_selector = MyOtherSelector::DeatachSelector(String::from("Custom Data"));
    // println!("Other selector detach: {other_selector:?}");

    match other_selector {
        MyOtherSelector::Choice1 => println!("You chose Choice 1"),
        MyOtherSelector::Choice2 => println!("You chose Choice 2"),
        MyOtherSelector::Choice3 => println!("You chose Choice 3"),
        MyOtherSelector::DeatachSelector(data) => println!("You detached with data: {data:?}"),
    }

    // Using match to handle different enum variants

    let val1 = Option::Some(10);
    let val2 : Option<i32> = Option::Some(20);

     // Pattern matching extracts the 10
    if let Option::Some(res_a) = val1 {
        // res_a now holds 10
        println!("{}", res_a); // prints: 10
    }

    println!("Value 1: {}",add(val1,val2));

    let a = Option::Some(10);
}

fn add(a: Option<i32>, b: Option<i32>) -> i32 {
    let res_a = if let Option::Some(res_a) = a {res_a} else {0};
    let res_b = if let Option::Some(res_b) = b {res_b} else {0};
    res_a + res_b
}