fn reverse_string(s: String) -> String {
    let mut char_array = s.chars().collect::<Vec<char>>();
    char_array.reverse();
    char_array.iter().collect::<String>()
}

fn main() {
    let s: String = String::from("Hello, world!");
    println!("{}", reverse_string(s));
}
