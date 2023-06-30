// https://leetcode.com/problems/distribute-candies-to-people/

/*

 i+n + 1

 2, 7, 12

 i*n + value

1 -> 1 + 6 + 11? (no)
2 -> 2 + 7?
3 -> 3
4 -> 4
5 -> 5

1+2+3+4+5= ive give 15, can I give 6 more? yes
1+2+3+4+...+10 = ive given 55, can I give 11 more? no

1+2+3+4+5+6= ive given 21, can I give 7 more? yes

*/

pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();

    for i in 0..num_people {
        let mut value = 0;
        let mut times_received_candies = 0;
        loop {
            let num_of_candies_to_be_given: i32 = times_received_candies * num_people + i + 1;
            let num_of_candies_given: f32 = ((num_of_candies_to_be_given - 1) as f32 / 2.0) * (1.0 + (num_of_candies_to_be_given - 1) as f32);

            if num_of_candies_given as i32 + num_of_candies_to_be_given > candies {
                break;
            }
            value += num_of_candies_to_be_given;
            times_received_candies += 1;
        }
        vector.push(value)
    }

    let sum: i32 = vector.iter().sum();

    let mut special_condition = true;
    for i in 0..num_people - 1 {
        let current = vector[i as usize];
        let next = vector[(i + 1) as usize];
        if next < current {
            vector[(i + 1) as usize] += candies - sum;
            special_condition = false;
            break;
        }
    }

    if special_condition {
        vector[0] += candies - sum;
    }

    vector
}

fn main() {
    let candies: i32 = 80;
    let num_people: i32 = 4;

    let solution: Vec<i32> = distribute_candies(candies, num_people);

    println!("{:?}", solution);
}
