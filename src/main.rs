mod math;

fn main() {
    let result = math::add(1, 2);
    let sub_result = math::sub(2, 1);
    println!("1 + 2 = {}", result);
    println!("2 - 1 = {}", sub_result);
}
