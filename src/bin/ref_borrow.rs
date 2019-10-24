fn main() {
    let mut s1 = String::from("hello"); // a mutable ref

    let _r1 = &mut s1;
    {
        let _r2 = &mut s1;
    }

    let len = calculate_length(&s1);
    change(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    {
        _main();
        fn _main() {
            // let reference_to_nothing = dangle();
            println!("{}", no_dangle())
        }

        // dangling pointer
        // fn dangle() -> &String {
        //     let s = String::from("hello");

        //     &s
        // }

        fn no_dangle() -> String {
            let s = String::from("hello");

            s
        }
    }
}

fn calculate_length(s: &String) -> usize {
    // borrowing: s is a reference to a String, refs are immutable by default
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// to borrow mutable ref
// mutable ref can only be borrowed once at a time in a scope
// in other words, at any given time, you can have either one mutable reference or any number of immutable references.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
