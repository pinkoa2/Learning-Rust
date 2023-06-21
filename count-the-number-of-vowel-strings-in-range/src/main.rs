pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let mut count: i32 = 0;
    for i in left..right + 1 {
        let u: usize = i as usize;

        let word = &words[u];

        let first_letter: char = word.chars().nth(0).expect("");
        let last_letter: char = word.chars().nth(word.len() - 1).expect("");

        if vowels.contains(&first_letter) && vowels.contains(&last_letter) {
            count += 1;
        }
    }
    return count;
}

fn main() {
    let input_vec: Vec<String> =
        Vec::from([String::from("are"), String::from("amy"), String::from("u")]);
    let left: i32 = 0;
    let right: i32 = 2;

    let response = vowel_strings(input_vec, left, right);

    println!("{response}");
}
