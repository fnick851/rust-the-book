fn main() {
    let x = 5;
    let mut y = 0;
    println!("x={} is immutable by default, y={} is mutable", x, y);
    y = 1;
    println!("y is mutated to {}", y);
    const Z: i32 = 2;
    println!("Z={} is a constant that never changes", Z);
    let x = "x";
    println!("x is shadowed to {}", x);
}
