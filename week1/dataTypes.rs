fn dataTypes() {
    //boolan
    let is_rust_fun = true;
    let is_rust_hard = false;
    //integer
    let x: i32 = 42;
    let min_i32 = i32::MIN;
    let max_i32 = i32::MAX;
   println!("The minimum value of i32 is {} and the maximum value is {}.", 
    min_i32, max_i32);
   //float
    let pi: f64 = 3.14159;
    //char
    let letter_a: char = 'a';
    //string
    let message: &str = "Hello, world!";
    let mut name = String::from("Alice");
    let numbers: [i32; 3] = [1, 2, 3];
    let second_number = numbers[1];
    println!("The second number in the array is {}.", second_number);
    //slices
    let slice = &numbers[1..3];
    let first_element = slice[0];
    println!("The first element of the slice is {}.", first_element);
    //tuples
    let person = ("Alice", 30);
    let name = person.0;
    let age = person.1;
    println!("The person's name is {} and their age is {}.", name, age);
    let person = (("Alice", "Smith"), 30);
    println!("The person's name is {} {} and their age is {}.", person.0.0, person.0.1, person.1);
    //unit type
    let result = do_something();
    println!("The result is {}.", result);
    let x: i32 = 42;

    let x = 42; // immutable variable
    x = 10; // error: cannot assign twice to immutable variable
    let mut x = 42; // mutable variable
    x = 10; // OK!
    let y = 3.14; // Rust infers the type as f64
    let x = 42;
    let x = x + 1;

    let _my_string=String::from("hi,simon");
    let _days_of_week: [&str;7]=[
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];
    let first_element=_days_of_week[0];
    let last_elemnt=_days_of_week[_days_of_week.len()-1];
}