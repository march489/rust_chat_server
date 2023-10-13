fn f(s: String) -> String {
    s
}

fn g(s: String) -> &'static str {
    //// version 1
    // s.leak()

    //// version 2
    Box::leak(s.into_boxed_str())
}

fn main() {
    let orig = String::from("hello");
    dbg!(f(orig.clone()));
    dbg!(g(orig.clone()));
}
