fn main() {
    let s: String = String::from("()");

    let mut vector: Vec<char> = Vec::new();

    for (_, token) in s.chars().enumerate() {
        match token {
            '(' | '[' | '{' => vector.push(token),
            _ => check_if_last_token_is_corresponding(&vector, token),
        }
    }
    println!("{:?}", vector);
}

fn check_if_last_token_is_corresponding(vector: &Vec<char>, token: char) {
    let last_token = vector.last(){
        Some(token) => token,
        None => '*'
    }
    println!("{:?}", last_token)
}
