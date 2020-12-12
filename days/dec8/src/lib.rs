use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::io;
use regex::{Regex};
use std::borrow::{Borrow, BorrowMut};

mod test;

#[derive(Debug, Clone)]
struct Instruction {
    pub name: String,
    pub value: i32
}

/*
    Returns the accumulator value before a jump to an already seen instruction
 */
fn solve_puzzle(path: &str) -> io::Result<i32> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut instructions: Vec<(Instruction, bool)> = Vec::new();
    let instruction_re = Regex::new(r#"([a-z]{3}) ([+-]\d+)$"#).unwrap();
    for line in reader.lines(){
        let the_line = line.unwrap();
        let captures = instruction_re.captures(the_line.borrow());
        if captures.is_some(){
            let cap = captures.unwrap();
            instructions.push(
                (Instruction{
                    name: cap[1].to_owned(),
                    value: cap[2].parse::<i32>().unwrap(),
                }, false)
            )
        }
    }

    let mut acc = 0;
    let (res, _pos) = interpret(0, &mut acc, true, &mut instructions);

    Ok(res as i32)
}

fn interpret(pos: i32, acc: &mut i32, stop_at_loop: bool, instructions: &mut Vec<(Instruction, bool)>) -> (i32, i32) {
    if pos >= instructions.len() as i32 {
        return (*acc, pos);
    }
    let instruction: &mut (Instruction, bool) = instructions.get_mut(pos as usize).unwrap();
    instruction.1 = true;


    match instruction.0.name.as_str() {
        "noop" => {},
        "jmp" => {
            let idx = pos + instruction.0.value;
            let peek: Option<&(Instruction, bool)> = instructions.get(idx as usize);
            if peek.is_some() {
                if peek.unwrap().1 {
                    return (*acc, pos);
                }
            }
            return interpret(idx, acc, stop_at_loop, instructions)}
        "acc" => {
            *acc += instruction.0.value
        }
        _ => {}
    }

    return interpret(pos + 1, acc, stop_at_loop, instructions);
}

fn solve_puzzle_part_b(path: &str) -> io::Result<i32> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut instructions: Vec<(Instruction, bool)> = Vec::new();
    let instruction_re = Regex::new(r#"([a-z]{3}) ([+-]\d+)$"#).unwrap();
    for line in reader.lines(){
        let the_line = line.unwrap();
        let captures = instruction_re.captures(the_line.borrow());
        if captures.is_some(){
            let cap = captures.unwrap();
            instructions.push(
                (Instruction{
                    name: cap[1].to_owned(),
                    value: cap[2].parse::<i32>().unwrap(),
                }, false)
            )
        }
    }

    let mut acc = 0;
    let mut arr_pos = 0;
    let mut cloned_instructions = instructions.clone();
    for i in cloned_instructions.iter_mut() {
        if i.0.name != "jmp" {
            arr_pos += 1;
            continue
        }

        {
            let jmp = instructions.get_mut(arr_pos).unwrap();
            jmp.0.name = "noop".parse().unwrap();
        }
        let (_res, pos) = interpret(0, &mut acc, true, instructions.borrow_mut());
        if pos < instructions.len() as i32 {
            {
                let jmp = instructions.get_mut(arr_pos).unwrap();
                jmp.0.name = "jmp".parse().unwrap();
            }

            reset(&mut instructions);
            acc = 0;
        } else {
            return Ok(acc);
        }
        arr_pos+=1;
    }

    Err(Error::new(ErrorKind::InvalidInput, "solution not found"))
}

fn reset(instructions: &mut Vec<(Instruction, bool)>) {
    for v in instructions.iter_mut(){
        v.1 = false;
    }
}