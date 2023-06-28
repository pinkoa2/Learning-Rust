pub fn height_checker(heights: Vec<i32>) -> i32 {

    let mut sorted_heights = heights.clone(); 
    sorted_heights.sort(); 

    let mut result = 0;
    for i in 0..heights.len() {
            if sorted_heights[i] != heights[i] {
                result += 1;
            }
    }
    return result;
}

fn main () {
    let input = vec![5,1,2,3,4];
    let solution = height_checker(input);
    println!("{solution}")
} 











