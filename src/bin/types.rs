fn main() {
    // scalar types
    println!("primary scalar types: integer, floating-point, boolean, character");
    println!("primitive compound types: tuple, array");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "coming from a tuple: {}, {}, {}, {}, {}, {}",
        x, y, z, five_hundred, six_point_four, one
    );
    let a = [3; 5];
    let b = [3, 3, 3, 3, 3];
    println!("two ways of writing the same array is {}", a == b);
}
