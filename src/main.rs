fn main() {
    println!("Control flow");

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("Condition ws false");
    }

    multiple_condition();

    if_in_let_statement();

    repetition_in_loop();

    multiple_loop();

    conditional_loop_with_while();
    looping_with_for();
}
fn multiple_condition() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is : {number}");
}

fn repetition_in_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("{counter}");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");
}

fn multiple_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remianing = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
fn conditional_loop_with_while() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40];

    let mut index = 0;

    while index < 4 {
        // this approach is error prone
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn looping_with_for() {
    // looping through a collection with for

    let b = [10, 20, 30, 40];

    for element in b {
        println!("the value in element is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
