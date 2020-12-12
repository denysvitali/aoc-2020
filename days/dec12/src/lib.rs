extern crate regex;
extern crate utils;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use regex::Regex;
use crate::Command::{North, South, East, West, Left, Right, Forward};

mod test;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

// Clockwise
const CLOCKWISE_DIRS: &[Direction] = &[
    Direction::North, Direction::East, Direction::South, Direction::West
];

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Command {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

/*
    Returns the Manhattan distance between the end position
    and the ship starting position
 */
fn solve_puzzle(path: &str,
                waypoint_pos: (i32, i32),
                waypoint_nav: bool,
) -> io::Result<i64> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut commands: Vec<(Command, i32)> = Vec::new();
    let cmd_re = Regex::new(r#"^([NSEWLRF])(\d+)$"#).unwrap();
    for line in reader.lines(){
        let the_line = line.unwrap();
        let cap = cmd_re.captures(the_line.as_str()).unwrap();
        let dir = (match cap.get(1).unwrap().as_str() {
            "N" => Some(North),
            "S" => Some(South),
            "E" => Some(East),
            "W" => Some(West),
            "L" => Some(Left),
            "R" => Some(Right),
            "F" => Some(Forward),
            _ => None
        }).unwrap();
        commands.push((dir, cap.get(2).unwrap().as_str().parse::<i32>().unwrap()));
    }

    let end_pos = interpret_commands(commands,
                                     waypoint_pos,
                                     &Direction::East,
                                     waypoint_nav);
    Ok(manhattan_distance((0,0), end_pos))
}

fn manhattan_distance(p: (i32, i32), q: (i32, i32)) -> i64 {
    return ((p.0 - q.0).abs() + (p.1 - q.1).abs()) as i64;
}

fn interpret_commands(commands: Vec<(Command, i32)>,
                      start: (i32, i32),
                      start_dir: &Direction,
                      waypoint_nav: bool) -> (i32, i32) {
    /*
        (0, 1)     N
       (-1, 0)   W X E   (1, 0)
        (0, -1)    S
     */
    let mut dir = *start_dir;
    let mut pos = (0,0);
    let mut waypoint = start;

    for i in commands {
        if waypoint_nav {
            match i.0 {
                North => {
                    add_pos(&mut waypoint, &get_rel_mult(&Direction::North, i.1));
                }
                South => {
                    add_pos(&mut waypoint, &get_rel_mult(&Direction::South, i.1));
                }
                East => {
                    add_pos(&mut waypoint, &get_rel_mult(&Direction::East, i.1));
                }
                West => {
                    add_pos(&mut waypoint, &get_rel_mult(&Direction::West, i.1));
                }
                Left => {
                    rot_wp(&mut waypoint, -i.1);
                }
                Right => {
                    rot_wp(&mut waypoint, i.1);
                }
                Forward => {
                    add_pos(&mut pos, &mult_pos(&waypoint, i.1));
                }
            }
        } else {
            match i.0 {
                Left => {
                    // Change dir
                    dir = get_new_dir(&dir, -i.1);
                    continue
                }
                Right => {
                    dir = get_new_dir(&dir, i.1);
                    continue
                }
                _ => {}
            }
            let rel_pos = get_relative_pos(&i.0, &dir);
            add_pos(&mut pos, &mult_pos(&rel_pos, i.1));
        }
    }
    return pos
}

fn rot_wp(wp: &mut (i32, i32), deg: i32) {
    let mut factor = deg/90;
    let cw = factor >= 0;
    factor = factor.abs();

    for _i in 1..=factor {
        if cw {
            *wp = (wp.1, -wp.0)
        } else {
            *wp = (-wp.1, wp.0)
        }
    }
}

fn get_rel_mult(dir: &Direction, factor: i32) -> (i32, i32) {
    let rel_pos = get_relative_pos(&dir2cmd(dir), dir);
    mult_pos(&rel_pos, factor)
}

fn mult_pos(pos: &(i32, i32), factor: i32) -> (i32, i32) {
    (pos.0 * factor, pos.1 * factor)
}

fn add_pos(pos: &mut (i32, i32), to: &(i32, i32)) {
    pos.0 += to.0;
    pos.1 += to.1;
}

fn get_new_dir(current_dir: &Direction, cw_deg: i32) -> Direction {
    let pos_idx = CLOCKWISE_DIRS.iter()
        .position(|d|d == current_dir).unwrap();

    let cwd_l = CLOCKWISE_DIRS.len();
    let idx = (pos_idx as i32 + (cw_deg/90) + cwd_l as i32) % cwd_l as i32;
    CLOCKWISE_DIRS.get(idx as usize).unwrap().clone()
}

fn get_relative_pos(command: &Command, direction: &Direction) -> (i32, i32) {
    match command {
        North => { (0, 1) },
        West => { (-1, 0) },
        East => { (1, 0) },
        South => { (0, -1) },
        Left => { (0, 0) }
        Right => { (0, 0) }
        Forward => { get_relative_pos(&dir2cmd(direction), direction)}
    }
}

fn dir2cmd(d: &Direction) -> Command {
    match d {
        Direction::North => Command::North,
        Direction::South => Command::South,
        Direction::East => Command::East,
        Direction::West => Command::West,
    }
}