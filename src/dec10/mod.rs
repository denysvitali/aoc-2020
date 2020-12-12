use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

// use num_integer::binomial;

mod test;

/*
    Returns the amount of 1-jolt differences * 3-jolt differences
 */
fn solve_puzzle(path: &str) -> io::Result<i64> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut nums: Vec<i64> = get_nums(reader);

    nums.sort();
    let output_jolts = nums.get(nums.len() - 1).unwrap() + 3;
    let mut diff_map: HashMap<i64, Vec<i64>> = HashMap::new();

    nums.push(output_jolts);

    let mut last_jolts = 0;
    for j in nums.iter() {
        let diff = j - last_jolts;
        if !diff_map.contains_key(&diff) {
            diff_map.insert(diff, Vec::new());
        }
        diff_map.get_mut(&diff).unwrap().push(*j);
        last_jolts = *j;
    }

    let res = (diff_map.get(&1).unwrap_or(&Vec::<i64>::new()).len() *
        diff_map.get(&3).unwrap_or(&Vec::<i64>::new()).len()) as i64;
    Ok(res)
}

fn solve_part_b(path: &str) -> io::Result<i64> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut adapters = get_nums(reader);
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    let combinations = find_combinations(adapters);
    Ok(combinations)
}

fn find_combinations(nums: Vec<i64>) -> i64 {
    let mut combinations: HashMap<i64, i64> = HashMap::new();
    combinations.insert(0, 1);
    inner_find_combinations(nums, &mut combinations)
}

fn inner_find_combinations(nums: Vec<i64>, combinations: &mut HashMap<i64, i64>) -> i64 {
    for i in &nums[1..] {
        let a = *combinations.get_mut(&(i-1)).unwrap_or(&mut 0);
        let b = *combinations.get_mut(&(i-2)).unwrap_or(&mut 0);
        let c = *combinations.get_mut(&(i-3)).unwrap_or(&mut 0);
        combinations.insert(*i, a+b+c);
    }
    return *combinations.get(nums.iter().max().unwrap()).unwrap();
}

fn get_nums(reader: BufReader<File>) -> Vec<i64> {
    let mut nums = Vec::new();
    for line in reader.lines() {
        let the_line = line.unwrap();
        let num = the_line.parse::<i64>().unwrap();
        nums.push(num);
    }
    return nums;
}
