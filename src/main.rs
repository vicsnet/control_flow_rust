fn main() {
    println!("Control flow");

    let number = 7;

    if number <5{
        println!("condition was true");

    }else{
        println!("Condition ws false");
    }

    multiple_condition();

    if_in_let_statement();
}
fn multiple_condition(){
    let number = 6;

    if number % 4 == 0{
        println!("number is divisible by 4");

    }else if number % 3 == 0{
        println!("number is divisible by 3");
    }else if number % 2 == 0{
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let_statement(){
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is : {number}");
}
