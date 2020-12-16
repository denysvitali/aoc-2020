mod test;

extern crate utils;
extern crate regex;

use std::io::{Error, BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

fn solve_puzzle(path: &str) -> Result<i32, Error> {
    let n = BufReader::new(File::open(path)?)
        .lines().map(|x|x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let re = Regex::new(r#"(?s)(.*?)\n\nyour ticket:\n(.*?)\n\nnearby tickets:\n(.*)"#).unwrap();
    let re_fields = Regex::new(r#"([A-z0-9 ]+): (\d+)-(\d+) or (\d+)-(\d+)"#).unwrap();
    let cap = re.captures(&n).unwrap();

    let fields = cap.get(1).unwrap().as_str()
        .split("\n").map(|x|get_field(x, &re_fields))
        .collect::<Vec<FieldRule>>();

    let my_ticket = cap.get(2).unwrap().as_str()
        .split(",").map(|x|x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let nearby_tickets = cap.get(3).unwrap().as_str()
        .split("\n")
        .map(|x| x.split(",")
                .map(|x|x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>();


    println!("fields={:?}\nmy_ticket={:?}\nnearby_tickets={:?}", fields, my_ticket, nearby_tickets);

    let mut invalid : Vec<i32> = Vec::new();
    for i in nearby_tickets {
        for v in i {
            let mut is_valid = false;
            for f in &fields {
                let fr = f.range.as_slice();
                if (fr[0] <= v && v <= fr[1]) || (fr[2] <= v && v <= fr[3]) {
                    is_valid = true;
                }
            }

            if !is_valid {
                invalid.push(v);
            }
        }
    }


    Ok(invalid.iter().sum())
}

#[derive(Debug)]
struct FieldRule {
    name: String,
    range: Vec<i32>
}

fn get_field(input: &str, re: &Regex) -> FieldRule {
    println!("input={}", input);
    let cap = re.captures(input).unwrap();
    return FieldRule{
        name: String::from(cap.get(1).unwrap().as_str()),
        range: vec![
            cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(5).unwrap().as_str().parse::<i32>().unwrap(),
        ]
    }
}