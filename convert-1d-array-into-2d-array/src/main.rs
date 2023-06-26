// https://leetcode.com/problems/convert-1d-array-into-2d-array/

pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    if original.len() != (m * n) as usize {
        return vec![];
    }

    let mut outer_layer_vector: Vec<Vec<i32>> = Vec::new();
    let mut inner_layer_vector: Vec<i32> = Vec::new();

    let mut i: usize = 0;
    let mut counter: usize = 0;
    while i < original.len() {
        inner_layer_vector.push(original[i]);
        counter += 1;
        if counter == n as usize {
            counter = 0;
            outer_layer_vector.push(inner_layer_vector);
            inner_layer_vector = Vec::new();
        }
        i += 1;
    }
    return outer_layer_vector;
}

fn main() {
    let input: Vec<i32> = vec![1, 2];
    let m = 1;
    let n = 1;

    let solution: Vec<Vec<i32>> = construct2_d_array(input, m, n);
    println!("{:?}", solution);
}
