fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let input_string = "главрыба";
    let reversed_string = reverse_string(input_string);
    println!("Исходная строка: {}", input_string);
    println!("Перевернутая строка: {}", reversed_string);
}