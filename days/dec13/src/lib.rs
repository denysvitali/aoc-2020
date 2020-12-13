use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

extern crate num_integer;

mod test;

fn solve_puzzle(path: &str) -> Result<i64, io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    // line 0: arrival ts
    // line 1: bus list

    let (arrival_ts, bus_list) = get_input(&mut lines);

    let mut bid = -1;
    let mut rem_min = i64::MAX;

    for i in bus_list {
        if i == -1 {
            continue
        }

        let t = i - arrival_ts % i;
        if t < rem_min {
            rem_min = t;
            bid = i;
        }
    }

    Ok(bid*rem_min)
}

fn get_input(mut lines: &mut Lines<BufReader<File>>) -> (i64, Vec<i64>) {
    let arrival_ts = get_arrival_ts(&mut lines);
    let bus_list = get_bus_list(&mut lines);
    (arrival_ts, bus_list)
}

fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1)
    }

    let (g, x, y) = xgcd(b % a, a);
    (g, y - (b / a) * x, x)
}

fn mod_inv(a: i64, b: i64) -> Option<i64> {
    let (g, x, _) = xgcd(a, b);
    if g != 1 {
        return None
    }
    Some((x % b + b) % b)
}

fn crt(residues: &[i64], moduli: &[i64]) -> Option<i64> {
    let prod = moduli.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(moduli) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

fn solve_puzzle_part_b(path: &str) -> Result<i64, io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    // line 0: arrival ts
    // line 1: bus list

    let _ = get_arrival_ts(&mut lines);
    let bus_list = get_bus_list(&mut lines);

    let diff = bus_list.iter().enumerate()
        .map(|x|if x.1 != &-1 { -(x.0 as i64) + x.1 } else { 1 as i64 })
        .filter(|x|x!=&1)
        .collect::<Vec<i64>>();

    let buses = bus_list.iter().enumerate()
        .map(|x|if x.1 != &-1 { *x.1 as i64 } else { -1 as i64 })
        .filter(|x|x!=&-1)
        .collect::<Vec<i64>>();

    let cr = crt(&diff[..], &buses[..]);
    return Ok(cr.unwrap());
}

fn get_arrival_ts(lines: &mut Lines<BufReader<File>>) -> i64 {
    lines.next().unwrap().unwrap().parse::<i64>().unwrap()
}

fn get_bus_list(lines: &mut Lines<BufReader<File>>) -> Vec<i64> {
    lines.next().unwrap().unwrap().split(",")
        .into_iter()
        .map(|x| if x == "x" { -1 } else { x.parse::<i64>().unwrap() })
        .collect::<Vec<i64>>()
}