mod mod_hosting;

pub fn eat_at_restaurant() {
    mod_hosting::add_to_waitlist();
    mod_hosting::add_to_waitlist();
    mod_hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
}
