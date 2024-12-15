use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sample_file_name = "sample_input.txt";

    let mut buf_reader =
        BufReader::new(File::open(sample_file_name).expect("Unable to open input file"));

    let mut line = String::new();

    let mut left_vector: Vec<i32> = Vec::new();
    let mut right_vector: Vec<i32> = Vec::new();

    while buf_reader.read_line(&mut line).unwrap() > 0 {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        left_vector.push(nums[0]);
        right_vector.push(nums[1]);

        line.clear();
    }

    left_vector.sort_unstable();
    right_vector.sort_unstable();

    let total_abs_diff: i32 = left_vector
        .iter()
        .zip(right_vector.iter())
        .map(|x| (x.0 - x.1).abs())
        .sum();

    assert_eq!(11, total_abs_diff);

    println!("Things looking good mate!");
}
