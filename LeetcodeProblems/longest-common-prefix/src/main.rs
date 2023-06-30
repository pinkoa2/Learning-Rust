fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut num_of_prefix = 0;
    let first_word: &String = &strs[0];
    if first_word == "" {
        return "".to_string();
    }
    'outer: loop {
        if first_word.len() < num_of_prefix {
            break 'outer;
        }
        let main_prefix = &first_word[..num_of_prefix];
        for word in &strs {
            if word.len() < num_of_prefix {
                break 'outer;
            }
            let prefix_of_word = &word[0..num_of_prefix];
            if main_prefix != prefix_of_word {
                break 'outer;
            }
        }
        num_of_prefix += 1;
        if num_of_prefix > 100 {
            break;
        }
    }
    if num_of_prefix == 0 {
        return "".to_string();
    }
    return first_word[..(num_of_prefix - 1)].to_string();
}

fn main() {
    let input: Vec<String> = Vec::from([String::from("flower")]);
    let result = longest_common_prefix(input);
    println!("{result}");
}
