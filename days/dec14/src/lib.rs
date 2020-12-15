use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::{Regex, Match};
use crate::InstructionType::{Mask, Mem};
use std::collections::HashMap;

mod test;

extern crate regex;
extern crate ansi_term;
extern crate itertools;

use ansi_term::Color::{Red, Blue};

enum InstructionType {
    Mask,
    Mem
}

struct Instruction {
    t: InstructionType,
    v: u64,
    v2: u64
}

struct InstructionV2 {
    t: InstructionType,
    v: u64,
    v2: u64,
    v3: u64
}

const DEBUG: bool = false;

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        if DEBUG{
            println!($($arg)*);
        }
    })
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
            let mask_v = get_mask_v(&mask_re, &the_line);
            let (v, v2, _) = get_masks(&mask_v);
            instructions.push(
                Instruction{ t: InstructionType::Mask, v, v2 }
            )
        } else {
            let cap = mem_re.captures(&the_line).unwrap();
            instructions.push(
                Instruction {
                    t: InstructionType::Mem,
                    v: must_get_u64(cap.get(1)),
                    v2: must_get_u64(cap.get(2)),
                }
            )
        }
    }

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = u64::MAX;

    for i in instructions {
        match i.t {
            Mask => { or_mask = i.v; and_mask = i.v2;
                debug!("or_mask=\t{:#038b} (= {0})\nand_mask=\t{:#038b} (= {1})", or_mask, and_mask);
            }
            Mem => {
                if !mem.contains_key(&i.v) {
                    mem.insert(i.v, 0);
                }

                let n = (i.v2 | or_mask) & and_mask;
                mem.insert(i.v, n);
                debug!("mem[{}]={:#038b} (= {})",i.v, n, n);
            }
        }
    }

    let sum = mem.iter()
        .map(|x|*x.1)
        .sum();



    Ok(sum)
}

fn must_get_u64(p0: Option<Match>) -> u64 {
    return p0.unwrap().as_str().parse::<u64>().unwrap()
}

fn get_mask_v<'a>(mask_re: &Regex, the_line: &'a String) -> &'a str {
    let cap = mask_re.captures(&the_line).unwrap();
    let mask_v = cap.get(1).unwrap().as_str();
    mask_v
}

fn display_u64(n: u64, one_is_x: bool) -> String {
    let bin_repr = format!("{:#038b}", n);
    let res = bin_repr.chars()
        .into_iter()
        .map(|x| if x == '0' {
            format!("{}", Blue.paint("0"))
        } else if x == '1' {
            if one_is_x {
                format!("{}", Red.paint("X"))
            } else {
                format!("{}", Red.paint("1"))
            }
        } else { format!("{}", x) })
        .collect::<String>();

    res
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
            let mask_v = get_mask_v(&mask_re, &the_line);
            let (v, v2, v3) = get_masks(&mask_v);
            instructions.push(
                InstructionV2{ t: InstructionType::Mask, v, v2, v3 }
            )
        } else {
            let cap = mem_re.captures(&the_line).unwrap();
            instructions.push(
                InstructionV2 {
                    t: InstructionType::Mem,
                    v: cap.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                    v2: cap.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                    v3: 0,
                }
            )
        }
    }

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut or_mask: u64 = 0;
    let mut floating_mask: u64 = u64::MAX;

    for inst in instructions {
        match inst.t {
            Mem => {
                debug!("addr_b\t\t\t=\t{} (= {})", display_u64(inst.v, false), inst.v);
                let n = inst.v | or_mask;
                debug!("n\t\t\t\t=\t{} (= {})", display_u64(n, false), n);

                mem.insert(n, inst.v2);

                let mut floating_pos = Vec::new();
                for i in 0..36 {
                    if 1 << i == (floating_mask & 1 << i) {
                        floating_pos.push(1 << i);
                    }
                }

                debug!("floating_pos={:?}", floating_pos);

                for i in 0..(2 as u64).pow(floating_pos.len() as u64 as u32) {
                    let s: u64 = floating_pos.iter()
                        .enumerate()
                        .filter(|(j,_)|i>>*j & 1 == 1)
                        .map(|(_j, x)|x)
                        .sum();

                    let the_num = (n & !floating_mask) + s;
                    debug!("\t{} (={})", display_u64(the_num, false), the_num);

                    mem.insert(the_num, inst.v2);
                }

                debug!("addr\t\t\t=\t{} (= {})", display_u64(n, false), n);
                debug!("mem[{}]\t\t\t=\t{}\n",n, display_u64(inst.v2, false));
            }
            Mask => { or_mask = inst.v; floating_mask = inst.v3;
                debug!("or_mask\t\t\t=\t{}", display_u64(or_mask, false));
                debug!("floating_mask\t=\t{}\n", display_u64(floating_mask, true));
            }
        }
    }

    let sum = mem.iter()
        .map(|x|*x.1)
        .sum();

    Ok(sum)
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