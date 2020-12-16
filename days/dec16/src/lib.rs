mod test;

extern crate utils;
extern crate regex;

use std::io::{Error, BufReader, BufRead};
use std::fs::File;
use std::collections::{HashMap};
use regex::Regex;

fn solve_puzzle(path: &str) -> Result<i32, Error> {
    let (fields, _, nearby_tickets) = get_tickets(path);

    let mut invalid : Vec<i32> = Vec::new();
    for nt in nearby_tickets {
        for t_val in nt {
            let mut is_valid = false;
            for f in &fields {
                let fr = f.range.as_slice();
                if validate_rule(t_val, fr) {
                    is_valid = true;
                }
            }

            if !is_valid {
                invalid.push(t_val);
            }
        }
    }


    Ok(invalid.iter().sum())
}

fn validate_rule(t_val: i32, fr: &[i32]) -> bool {
    return (fr[0] <= t_val && t_val <= fr[1]) || (fr[2] <= t_val && t_val <= fr[3])
}

fn solve_puzzle_part_2(path: &str) -> Result<i64, Error> {
    let (fields, my_ticket, nearby_tickets) = get_tickets(path);
    let valid_tickets = get_valid_tickets(&fields, nearby_tickets);

    let mut candidates: HashMap<usize, HashMap<String, i32>> = HashMap::new();

    println!("valid_tickets={:?}", valid_tickets);

    for ticket in &valid_tickets {
        for (i, v) in ticket.iter().enumerate() {
            for f in &fields {
                let fr = f.range.as_slice();
                if (fr[0] <= *v && *v <= fr[1]) || (fr[2] <= *v && *v <= fr[3]) {
                    if !candidates.contains_key(&i) {
                        candidates.insert(i, HashMap::new());
                    }

                    let f_hm = candidates.get_mut(&i).unwrap();
                    if !f_hm.contains_key(&f.name) {
                        f_hm.insert(String::from(&f.name), 1);
                    } else {
                        f_hm.insert(String::from(&f.name), f_hm.get(&f.name).unwrap() + 1);
                    }
                }
            }
        }
    }

    println!("candidates={:?}", candidates);
    println!("valid_tickets={:?}", valid_tickets);
    let mut can_be: HashMap<usize, Vec<String>> = HashMap::new();

    for (k, v) in candidates {
        let max= *v.iter().map(|x|x.1).max().unwrap();
        for (k2, v2) in v {
            if v2 == max {
                if !can_be.contains_key(&k) {
                    can_be.insert(k, Vec::new());
                }
                can_be.get_mut(&k).unwrap().push(k2);
            }
        }
    }

    println!("can_be={:?}", can_be);

    /*
           0     1     2
           R    CRS    RC


            0     1     2
        R   1     1     1
        C   0     1     1
        S   0     1     0
     */

    let mut zeroed_vec = Vec::new();
    for _ in 0..fields.len() {
        zeroed_vec.push(false);
    }

    let mut matrix: HashMap<String, Vec<bool>> = HashMap::new();
    for (idx, possible_fields) in can_be {
        for f in &possible_fields {
            if !matrix.contains_key(f) {
                matrix.insert(f.to_string(), zeroed_vec.clone());
            }

            let ms = matrix.get_mut(f).unwrap().as_mut_slice();
            ms[idx] = true;
        }
    }

    println!("matrix={:?}", matrix);

    let mut field_mapping : HashMap<usize, String> = HashMap::new();

    loop {
        if field_mapping.len() == fields.len() {
            break
        }
        let mut found = None;
        let mut found_at = 0;
        for (k, v) in &matrix {
            let mut count_true = 0;
            let mut last_idx = 0;
            for (idx, value) in v.iter().enumerate() {
                if *value == true {
                    count_true += 1;
                    last_idx = idx;
                }
            }

            if count_true == 1{
                println!("{} can only be in {}", k, last_idx);
                field_mapping.insert(last_idx, String::from(k));
                found = Some(k.clone());
                found_at = last_idx;
                break
            }
        }

        if found.is_some() {
            matrix.remove(&found.unwrap());
            for (_, v) in matrix.iter_mut() {
                *v.get_mut(found_at).unwrap() = false;
            }
        }
    }

    let mut mult: i64 = 1;
    let mt_s = my_ticket.as_slice();

    for (k, v) in field_mapping {
        if v.starts_with("departure") {
            mult *= mt_s[k] as i64;
        }
    }

    Ok(mult)
}

fn get_valid_tickets(fields: &Vec<FieldRule>, nearby_tickets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut valid_tickets = Vec::new();
    for nt in nearby_tickets {
        let mut is_valid_ticket = true;
        for t_val in &nt {
            let mut is_valid = false;
            for f in fields {
                if validate_rule(*t_val, f.range.as_slice()) {
                    is_valid = true;
                    break;
                }
            }
            if !is_valid {
                is_valid_ticket = false;
            }
        }

        if is_valid_ticket {
            valid_tickets.push(nt);
        }
    }
    valid_tickets
}


fn get_tickets(path: &str) -> (Vec<FieldRule>, Vec<i32>, Vec<Vec<i32>>) {
    let n = BufReader::new(File::open(path).unwrap())
        .lines().map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let re = Regex::new(r#"(?s)(.*?)\n\nyour ticket:\n(.*?)\n\nnearby tickets:\n(.*)"#).unwrap();
    let re_fields = Regex::new(r#"([A-z0-9 ]+): (\d+)-(\d+) or (\d+)-(\d+)"#).unwrap();
    let cap = re.captures(&n).unwrap();

    let fields = cap.get(1).unwrap().as_str()
        .split("\n").map(|x| get_field(x, &re_fields))
        .collect::<Vec<FieldRule>>();

    let my_ticket = cap.get(2).unwrap().as_str()
        .split(",").map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let nearby_tickets = cap.get(3).unwrap().as_str()
        .split("\n")
        .map(|x| x.split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>();
    (fields, my_ticket, nearby_tickets)
}



#[derive(Debug)]
struct FieldRule {
    name: String,
    range: Vec<i32>
}

fn get_field(input: &str, re: &Regex) -> FieldRule {
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