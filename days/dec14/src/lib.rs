extern crate ansi_term;
extern crate itertools;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use regex::{Match, Regex};

use crate::InstructionType::{Mask, Mem};

mod test;

enum InstructionType {
    Mask,
    Mem,
}

struct Instruction {
    t: InstructionType,
    v: u64,
    v2: u64,
}

struct InstructionV2 {
    t: InstructionType,
    v: u64,
    v2: u64,
    v3: u64,
}

fn solve_puzzle(path: &str) -> Result<u64, io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mask_re = Regex::new(r#"^mask = ([X01]+)$"#).unwrap();
    let mem_re = Regex::new(r#"mem\[(\d+)] = (\d+)$"#).unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    for i in lines {
        let the_line = i.unwrap();
        if mask_re.is_match(&the_line) {
            let (v, v2, _) = get_masks(&get_mask_v(&mask_re, &the_line));
            instructions.push(Instruction { t: InstructionType::Mask, v, v2 })
        } else {
            let cap = mem_re.captures(&the_line).unwrap();
            instructions.push(Instruction {
                t: InstructionType::Mem,
                v: must_get_u64(cap.get(1)),
                v2: must_get_u64(cap.get(2))
            })
        }
    }

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = u64::MAX;

    for i in instructions {
        match i.t {
            Mask => { or_mask = i.v; and_mask = i.v2; }
            Mem => { mem.insert(i.v, (i.v2 | or_mask) & and_mask); }
        }
    }
    Ok(mem.iter().map(|x| *x.1).sum())
}

fn must_get_u64(p0: Option<Match>) -> u64 {
    return p0.unwrap().as_str().parse::<u64>().unwrap();
}

fn get_mask_v<'a>(mask_re: &Regex, the_line: &'a String) -> &'a str {
    let cap = mask_re.captures(&the_line).unwrap();
    let mask_v = cap.get(1).unwrap().as_str();
    mask_v
}

fn solve_puzzle_part_b(path: &str) -> Result<u64, io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mask_re = Regex::new(r#"^mask = ([X01]+)$"#).unwrap();
    let mem_re = Regex::new(r#"mem\[(\d+)] = (\d+)$"#).unwrap();
    let mut instructions: Vec<InstructionV2> = Vec::new();
    for i in lines {
        let the_line = i.unwrap();
        if mask_re.is_match(&the_line) {
            let (v, v2, v3) = get_masks(&get_mask_v(&mask_re, &the_line));
            instructions.push(InstructionV2 { t: InstructionType::Mask, v, v2, v3 })
        } else {
            let cap = mem_re.captures(&the_line).unwrap();
            instructions.push(InstructionV2 {
                t: InstructionType::Mem,
                v: cap.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                v2: cap.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                v3: 0,
            })
        }
    }

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut or_mask: u64 = 0;
    let mut floating_mask: u64 = u64::MAX;

    for inst in instructions {
        match inst.t {
            Mem => {
                let mut floating_pos = Vec::new();
                for i in 0..36 {
                    if 1 << i == (floating_mask & 1 << i) {
                        floating_pos.push(1 << i);
                    }
                }
                for i in 0..(2 as u64).pow(floating_pos.len() as u64 as u32) {
                    let s: u64 = floating_pos.iter()
                        .enumerate()
                        .filter(|(j, _)| i >> *j & 1 == 1)
                        .map(|(_j, x)| x)
                        .sum();
                    mem.insert(((inst.v | or_mask) & !floating_mask) + s, inst.v2);
                }
            }
            Mask => {
                or_mask = inst.v;
                floating_mask = inst.v3;
            }
        }
    }
    Ok(mem.iter().map(|x| *x.1).sum())
}

fn get_masks(mask_v: &&str) -> (u64, u64, u64) {
    let v = get_or_mask(&mask_v);
    let v2 = get_and_mask(&mask_v);
    let v3 = get_floating_mask(&mask_v);
    (v, v2, v3)
}

fn get_floating_mask(mask_v: &&str) -> u64 {
    u64::from_str_radix(&mask_v.chars().into_iter()
        .map(|x| if x == 'X' { '1' } else { '0' })
        .collect::<String>(), 2).unwrap()
}

fn get_and_mask(mask_v: &&str) -> u64 {
    u64::from_str_radix(&mask_v.chars().into_iter()
        .map(|x| if x != '0' { '1' } else { '0' })
        .collect::<String>(), 2).unwrap()
}

fn get_or_mask(mask_v: &&str) -> u64 {
    u64::from_str_radix(&mask_v.chars().into_iter()
        .map(|x| if x != '1' { '0' } else { '1' })
        .collect::<String>(), 2).unwrap()
}