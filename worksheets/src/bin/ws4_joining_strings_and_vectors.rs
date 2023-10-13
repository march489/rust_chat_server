fn join_ref_str_array() {
    let array: [&str; 4] = ["the", "rain", "in", "Spain"];
    let slice: &[&str] = &array;
    let s: String = slice.join(" ");
    println!("{s}");
}

fn join_vector_slice() {
    let array: [&[i32]; 3] = [&[1, 2], &[3, 4, 5], &[6, 7, 8, 9]];
    let slice: &[&[i32]] = &array;
    let v: Vec<i32> = slice.join::<&[i32]>(&[-2, -1]);
    println!("{v:?}");
}

fn join_string_slice() {
    let array: [String; 3] = [
        "January".to_string(),
        "February".to_string(),
        "March".to_string(),
    ];
    let slice: &[String] = &array;
    let s = slice.join(", ");
    println!("{s}");
}

fn join_vec_string() {
    let v: Vec<String> = vec![
        "Whether".to_string(),
        "the".to_string(),
        "weather".to_string(),
        "be".to_string(),
        "cold".to_string(),
    ];
    let result: String = v.join(" ");
    println!("{result}");
}

fn join_ref_str_vec() {
    let arr: Vec<&str> = vec!["April", "is", "the", "cruelest", "month"];
    let result = arr.join(" ");
    println!("{result}");
}

fn main() {
    join_ref_str_array();
    join_vector_slice();
    join_string_slice();
    join_vec_string();
    join_ref_str_vec()
}
