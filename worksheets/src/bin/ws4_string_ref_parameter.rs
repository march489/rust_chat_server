fn f(r: &String) -> &str {
    &r[2..]
}

fn main() {
    // make a string and get its ref
    dbg!(f(&("hello".to_string())));
}
