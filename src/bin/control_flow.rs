fn main() {
    // if
    let number = 6;
    let _unused_number = if number % 4 == 0 {
        println!("number is divisible by 4");
        number
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
        number
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
        number
    } else {
        println!("number is not divisible by 4, 3, or 2");
        number
    };

    // loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while
    let mut num = 3;

    while num != 0 {
        println!("{}!", num);

        num -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
