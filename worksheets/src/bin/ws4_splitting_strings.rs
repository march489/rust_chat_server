fn f1(r: &str) -> [&str; 3] {
    [&r[0..4], &r[4..8], &r[8..]]
}

// fn f2 (r : &str) -> &[&str] {
//   ...
// }

fn f3(r: &str) -> Vec<&str> {
    vec![&r[0..4], &r[4..8], &r[8..]]
}

fn g1(r: &str) -> [String; 3] {
    [r[0..4].to_string(), r[4..8].to_string(), r[8..].to_string()]
}

// fn g2 (r : &str) -> &[String] {
//   ...
// }

fn g3(r: &str) -> Vec<String> {
    vec![r[0..4].to_string(), r[4..8].to_string(), r[8..].to_string()]
}

fn main() {
    dbg!(f1("the rain in Spain"));
    // dbg!(f2("the rain in Spain"));
    dbg!(f3("the rain in Spain"));
    dbg!(g1("the rain in Spain"));
    // dbg!(g2("the rain in Spain"));
    dbg!(g3("the rain in Spain"));
}
