use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::io;
use std::borrow::{Borrow, BorrowMut};

mod test;

/*
    Returns the outlier that doesn't respect the rule
 */
fn solve_puzzle(path: &str, preamble_size: i32) -> io::Result<i64> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut preamble: Vec<i64> = Vec::new();
    let mut nums: Vec<i64> = Vec::new();
    let mut line_nr = 0;
    for line in reader.lines(){
        let the_line = line.unwrap();
        let num = the_line.parse::<i64>().unwrap();
        if line_nr < preamble_size {
            preamble.push(num);
        } else {
            nums.push(num)
        }
        line_nr += 1;
    }

    let mut preamble_sum: Vec<i64> = Vec::new();
    compute_preamble_sum(preamble.borrow(), preamble_sum.borrow_mut());

    for n in nums {
        if !preamble_sum.iter().any(|&x|x==n) {
            return Ok(n)
        }

        // Remove first element, push next one (n)
        preamble = Vec::from(&preamble[1..preamble_size as usize]);
        preamble.push(n);
        compute_preamble_sum(preamble.borrow(), preamble_sum.borrow_mut());
    }

    Err(Error::new(ErrorKind::InvalidInput,
                   "all numbers are summing to something in preamble_sum")
    )
}

fn solve_part_b(path: &str, preamble_size: i32) -> io::Result<i64> {
    let target = solve_puzzle(path, preamble_size).unwrap();
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut nums = Vec::new();

    for line in reader.lines(){
        let l = line.unwrap();
        let n = l.parse::<i64>().unwrap();
        nums.push(n);
    }

    for i in 0..nums.len() {
        let mut sum = 0;
        let mut max = 0;
        let mut min = i64::MAX;
        let mut num_set = Vec::new();
        for j in i+1..nums.len(){
            let new_num = nums.get(j).unwrap();
            if *new_num < min {
                min = *new_num;
            }
            if *new_num > max {
                max = *new_num;
            }

            if sum + new_num < target {
                num_set.push(new_num);
                sum += new_num;
            } else if sum + new_num > target {
                break
            } else {
                // Found it!
                println!("range = {} - {}, {:?}, {}", i, j, &nums[i..j], target);
                println!("smallest = {}", min);
                println!("largest = {}", max);
                return Ok(min + max)
            }
        }
    }



    Err(Error::new(ErrorKind::InvalidInput,
                   "all numbers are summing to something in preamble_sum")
    )
}


fn compute_preamble_sum(preamble: &Vec<i64>, preamble_sum: &mut Vec<i64>) {
    let preamble_size = preamble.len();
    *preamble_sum = Vec::new();
    for i in 0..preamble_size {
        for j in i+1..preamble_size {
            preamble_sum.push(
                preamble.get(i as usize).unwrap() +
                    preamble.get(j as usize).unwrap()
            );
        }
    }
}