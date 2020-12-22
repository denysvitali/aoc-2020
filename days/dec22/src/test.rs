use utils::get_file;

use crate::solve_puzzle;

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