fn main() {
    let args = std::env::args().skip(1);

    let result = args
        .map(|s| s.parse::<i32>().unwrap())
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("{}", result);
}
