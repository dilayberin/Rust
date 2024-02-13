fn controlFlow(){
    let x = 5;

    if x > 10 {
        println!("x is greater than 10");
    } else if x < 10 {
        println!("x is less than 10");
    } else {
        println!("x is equal to 10");
    } 

    while condition {
        // Code to execute
    }
    
    //Here's an example of a while loop in Rust:
    let mut counter = 0;
    
    while counter < 5 {
        println!("Counter value is {}", counter);
        counter += 1;
    }

    for item in collection {
        // Code to execute }
    
    //Here's an example of a for loop in Rust:
    let numbers = vec![1, 2, 3, 4, 5];
    
    for number in numbers {
        println!("Number is {}", number);
    }

    loop {
        // Code to execute     if condition {
            break;
        }
    }

    let mut counter = 0;

loop {
    println!("Counter value is {}", counter);
    counter += 1;

    if counter == 5 {
        break;
    }
}

for num in 1..5{
    println!("number is {}",num);
}

}