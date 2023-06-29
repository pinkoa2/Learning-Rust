use std::cmp;

fn smallest_dist(l1: char, l2: char) -> i32 {

    let i1: i32 = l1 as i32;
    let i2: i32 = l2 as i32;

    let clockwise = i32::abs(i1 - i2);
    let counterclockwise = i32::abs(clockwise - 26);


    return cmp::min(clockwise, counterclockwise)
}


pub fn min_time_to_type(word: String) -> i32 {

    let mut result: i32 = word.len() as i32;
    let mut chars: Vec<char> = word.chars().collect();
    chars.insert(0, 'a');
    for i in 0..chars.len()-1 {
        result += smallest_dist(chars[i as usize], chars[ i as usize + 1]);
    }

    return result
}


fn main() {
    let input = String::from("zjpc");
    let solution = min_time_to_type(input);
    println!("{solution}");

    
}
