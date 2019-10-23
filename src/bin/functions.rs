fn main() {
    new_func(2);
    println!("{}", new_func(2))
}

fn new_func(x: i32) -> i32 {
    println!("this is the new_func printing number {}", x);

    let y = {
        let z = 1;
        x + z // this is an expression, adding `;` turns it into statement
    }; // {} is an expression

    println!("The value of y is: {}", y);
    3 // function auto return the last expression
}
