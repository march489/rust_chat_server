fn f(r: &str) -> &str {
    &r[2..]
}

fn f1() -> &'static str {
    //needs 'static lifetime
    "hello"
}

fn f2(s: String) -> &'static str {
    // needs 'static lifetime
    s.leak()
}

fn f3(r: &String) -> &str {
    &r[..]
}

fn f4(r: &str) -> &str {
    r // do nothing
}

fn main() {
    dbg!(f("hello"));
    dbg!(f(&String::from("hello")));
    dbg!(f1());
    dbg!(f2("What?".to_string()));
    dbg!(f3(&("Who?".to_string())));
    dbg!(f4("Where?"));
}
