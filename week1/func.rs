fn func(){
    fn function_name(parameter1: type1, parameter2: type2) -> return_type {
        // Function body
        return value;
    }
    //fonk içinde return 1; demeden 1 dersek aynı anlama gelir
    fn add_numbers(x: i32, y: i32) -> i32//dönüş tipini belirtir
     {
        let result = x + y;
        return result;
    }
    fn get_greeting() -> String {
        return String::from("Hello, Rust!");
    }
    let sum = add_numbers(3, 5);
    fn greet(name: Option<&str>) {
        match name {
            Some(n) => println!("Hello, {}!", n),
            None => println!("Hello, Rust!"),
        }
    }
    fn get_greeting(name: &str) -> String {
        let greeting = format!("Hello, {}!", name);
        return greeting;
    }
}