use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::io;
use std::num::ParseIntError;

mod test;

const EXPECTED_SUM: i32 = 2020;

fn solve_puzzle(path: &str) -> io::Result<i32> {
    // Parse each line and get an array of ints
    let int_vec = get_ints(path);
    let m_i_vec = int_vec?.to_owned();

    for i in 0..m_i_vec.len() {
        for j in (i + 1)..m_i_vec.len() {
            let i_num = m_i_vec.get(i).unwrap();
            let j_num = m_i_vec.get(j).unwrap();
            let sum = i_num + j_num;

            if sum == EXPECTED_SUM {
                return Ok(i_num * j_num);
            }
        }
    }
    Err(Error::new(ErrorKind::InvalidInput,
                   format!(
                       "Input doesn't contain any number that sums to {}",
                       EXPECTED_SUM
                   )
    ))
}

fn get_ints(path: &str) -> io::Result<Vec<i32>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut int_vec: Vec<i32> = vec!();
    for line in reader.lines() {
        let mut error: Option<ParseIntError> = None;
        let int = match line?.parse::<i32>() {
            Ok(n) => {
                n
            }
            Err(e) => {
                error = Some(e);
                0
            }
        };

        if error.is_some() {
            return Err(Error::new(ErrorKind::InvalidInput, "Input contains a non-int"));
        }

        int_vec.push(int);
    }

    Ok(int_vec)
}