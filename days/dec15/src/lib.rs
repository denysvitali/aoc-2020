mod test;

extern crate utils;

use std::io::{Error, BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn solve_puzzle(path: &str, target: i32) -> Result<i64, Error> {
    let n = BufReader::new(File::open(path)?)
        .lines().map(|x|x.unwrap()).collect::<Vec<_>>()
        .get(0).unwrap().split(",")
        .map(|x|x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    println!("nums={:?}", n);

    let mut seen: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut rec_spoken = -1;
    let mut first_time = false;

    for (i, v) in n.iter().enumerate() {
        speak(v, (i as i64) +1, &mut seen, &mut rec_spoken, &mut first_time);
    }

    let empty : Vec<i64> = Vec::new();
    let start : i64 = n.len() as i64 + 1;
    let end : i64 = (target as i64 + 1);
    for i in start..end {
        let s = seen.get(&rec_spoken).unwrap_or(&empty);
        if s.len() != 2 {
            speak(&0, i, &mut seen, &mut rec_spoken, &mut first_time)
        } else {
            speak(&(s.get(1).unwrap() - s.get(0).unwrap()), i, &mut seen, &mut rec_spoken, &mut first_time);
        }
    }

    Ok(rec_spoken)
}

fn speak(nr: &i64, i: i64, seen: &mut HashMap<i64, Vec<i64>>, rec_spoken: &mut i64, first_time: &mut bool) {
    *first_time = !seen.contains_key(nr);
    if !*first_time {
        let v = seen.get(nr).unwrap();
        seen.insert(*nr, vec![*v.last().unwrap(), i as i64]);
    } else {
        seen.insert(*nr, vec![i as i64]);
    }
    *rec_spoken = *nr;
}