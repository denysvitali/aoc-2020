use utils::get_file;

use crate::{solve_puzzle, solve_puzzle_2};

#[test]
fn example(){
    let path = get_file(file!(), "1.txt");
    assert_eq!(306, solve_puzzle(&path).unwrap())
}

#[test]
fn example_2(){
    let path = get_file(file!(), "3.txt");
    assert_eq!(306, solve_puzzle(&path).unwrap())
}

#[test]
fn puzzle(){
    let path = get_file(file!(), "2.txt");
    assert_eq!(306, solve_puzzle(&path).unwrap())
}

#[test]
fn example_1_part_2(){
    let path = get_file(file!(), "4.txt");
    assert_eq!(306, solve_puzzle_2(&path).unwrap())
}