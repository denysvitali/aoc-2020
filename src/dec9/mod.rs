use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::io;
use std::borrow::{Borrow, BorrowMut};
// use num_integer::binomial;

mod test;

/*
    Returns the accumulator value before a jump to an already seen instruction
 */
fn solve_puzzle(path: &str, preamble_size: i32) -> io::Result<i64> {
    println!("path={}, preamble_size={}", path, preamble_size);
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


    println!("nums={:?}, path={}", nums, path);

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



    // let slice_size = binomial(preamble_size, 1);
    // let preamble_combination_size = binomial(preamble_size, 2);
    // println!("slice_size={}", slice_size);
    //
    // for (i, n) in nums.iter().enumerate() {
    //     println!("iter: {}, {}", i, n);
    //     if !preamble_sum.iter().any(|&x|x==*n) {
    //         return Ok(*n)
    //     }
    //     // New preamble_sum will be [1 preamble_size + 1]
    //     println!("preamble_sum before = {:?}, len = {}", preamble_sum, preamble_sum.len());
    //     preamble_sum = Vec::from(&preamble_sum[(slice_size as usize)-1..(preamble_combination_size as usize)]);
    //     println!("preamble_sum after = {:?}, len = {}", preamble_sum, preamble_sum.len());
    //     preamble = Vec::from(&preamble[1..preamble_size as usize]);
    //
    //     let prev_num = *nums.get(i).unwrap();
    //     for j in preamble.iter() {
    //         println!("{} + {} = {}", prev_num, j, prev_num + j);
    //         preamble_sum.push(prev_num + j);
    //     }
    //     preamble.push(prev_num);
    //     println!("preamble_sum after_2 = {:?}, len = {}", preamble_sum, preamble_sum.len());
    // }

    Err(Error::new(ErrorKind::InvalidInput,
                   "all numbers are summing to something in preamble_sum")
    )
}

fn solve_part_b(path: &str, preamble_size: i32) -> io::Result<i64> {
    let target = solve_puzzle(path, preamble_size).unwrap();
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut nums = Vec::new();

    println!("target = {}", target);

    for line in reader.lines(){
        let l = line.unwrap();
        let n = l.parse::<i64>().unwrap();
        nums.push(n);
    }

    for i in 0..nums.len() {
        let mut sum = 0;
        let mut num_set = Vec::new();
        for j in i..nums.len(){
            let new_num = nums.get(j).unwrap();
            if sum + new_num < target {
                num_set.push(new_num);
                sum += new_num;
            } else if sum + new_num > target {
                break
            } else {
                // Found it!
                println!("range = {} - {}, {:?}, {}", i, j, &nums[i..j], target);
                let smallest = *nums[i..j].iter().min().unwrap();
                let largest = *nums[i..j].iter().max().unwrap();
                println!("smallest = {}", smallest);
                println!("largest = {}", largest);
                return Ok(smallest + largest)
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