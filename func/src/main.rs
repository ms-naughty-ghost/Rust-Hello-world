// https://doc.rust-jp.rs/book-ja/ch03-03-how-functions-work.html
fn main() {
    let response = print_labeled_measurement(5, 'h');
    println!("response:{}", response);
}

fn print_labeled_measurement(value: i32, unit_label: char) -> String {
    // println!("The measurement is: {}{}", &value, unit_label);
    format!("{}{}", &value, unit_label)
}
