fn main() {
    println!("Control flow");

    let number = 7;

    if number <5{
        println!("condition was true");

    }else{
        println!("Condition ws false");
    }

    multiple_condition();
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
