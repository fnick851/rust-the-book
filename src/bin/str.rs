fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("s is {}", s);

    // let s2 = &hello[0..1]; // this will crash
    // println!("s2 is {}", s2);
}
