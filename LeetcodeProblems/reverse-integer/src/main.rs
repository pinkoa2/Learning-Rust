fn reverse(x: i32) -> i32 {
    let string: String = x.abs().to_string();
    let reversed_string: String = string.chars().rev().collect();
    let value: i32 = match reversed_string.parse::<i32>() {
        Ok(result) => result,
        Err(_) => 0,
    };
    if x < 0 {
        return value * -1;
    }
    return value;
}

fn main() {
    let input: i32 = 1534236469;
    let solution: i32 = reverse(input);
    println!("{solution}")
}
