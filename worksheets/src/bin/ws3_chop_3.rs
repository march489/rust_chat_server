fn chop_3(s: String) -> Vec<[char; 3]> {
    let mut tmp = Vec::<char>::new();
    let mut result = Vec::<[char; 3]>::new();
    let mut ticker = 0;

    for ch in s.chars() {
        tmp.push(ch);
        ticker += 1;

        if 3 == ticker {
            result.push([tmp[0], tmp[1], tmp[2]]);
            tmp.clear();
            ticker = 0;
        }
    }

    if !tmp.is_empty() {
        while ticker < 3 {
            tmp.push('\0');
            ticker += 1;
        }
        result.push([tmp[0], tmp[1], tmp[2]]);
    }

    result
}

fn main() {
    let s: String = String::from("Hello, world!");
    println!("{:?}", chop_3(s.clone()));
}
