use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::borrow::Borrow;

mod test;

const DEBUG : bool = false;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum SeatStatus {
    Occupied,
    Empty,
    Floor
}

/*
    Returns the number of occupied seats once people stop moving around
 */
fn solve_puzzle(path: &str, min_adj: i32) -> io::Result<i64> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut pos: Vec<Vec<SeatStatus>> = Vec::new();
    for line in reader.lines(){
        let mut row: Vec<SeatStatus> = Vec::new();
        for x in line.unwrap().chars().into_iter() {
            row.push((match x {
                'L' => Some(SeatStatus::Empty),
                '#' => Some(SeatStatus::Occupied),
                '.' => Some(SeatStatus::Floor),
                _ => None
            }).unwrap())
        }
        pos.push(row);
    }

    let last_result : Vec<Vec<SeatStatus>>;
    loop {
        let (result, changed) = do_iter(pos.borrow(), min_adj);
        if DEBUG {
            println!("{}", pretty_result(&result));
        }
        if !changed {
            last_result = result;
            break
        }
        pos = result;
    }

    return Ok(last_result.iter()
        .map(|x|x.iter().filter(|s|**s == SeatStatus::Occupied))
        .flatten()
        .count() as i64);
}

fn pretty_result(pos: &Vec<Vec<SeatStatus>>) -> String {
    let mut sb : String = String::new();
    for y in pos {
        for x in y {
            sb.push(match x {
                SeatStatus::Empty => "L",
                SeatStatus::Occupied => "#",
                SeatStatus::Floor => "."
            }.parse().unwrap())
        }
        sb.push("\n".parse().unwrap())
    }
    sb
}

fn do_iter(pos: &Vec<Vec<SeatStatus>>, min_adj: i32) -> (Vec<Vec<SeatStatus>>, bool) {
    let mut new_pos : Vec<Vec<SeatStatus>> = Vec::new();
    let mut changed = false;
    for (y, row) in pos.iter().enumerate() {
        let mut new_row: Vec<SeatStatus> = Vec::new();
        for (x, s) in row.iter().enumerate() {
            let adj = count_adj(&pos, x, y, match min_adj {
                4 => false,
                _ => true
            });
            if adj == 0 && *s == SeatStatus::Empty {
                new_row.push(SeatStatus::Occupied);
                changed = true;
            } else if *s == SeatStatus::Occupied && adj >= min_adj {
                new_row.push(SeatStatus::Empty);
                changed = true;
            } else {
                let seat_status = s.clone();
                new_row.push(seat_status);
            }
        }
        new_pos.push(new_row);
    }

    (new_pos, changed)
}

fn pos_at(pos: &Vec<Vec<SeatStatus>>, x: i32, y: i32) -> &SeatStatus {
    return pos.get(y as usize).unwrap().get(x as usize).unwrap();
}

fn is_valid_pos(x: i32, y: i32, limit_x: i32, limit_y: i32) -> bool {
    x >= 0 && x < limit_x && y >= 0 && y < limit_y
}

/* 
    Count adjacent occupied seats
*/
fn count_adj(pos: &Vec<Vec<SeatStatus>>, x: usize, y: usize, consider_full_line: bool) -> i32 {
    /*
        XXX
        XOX
        XXX
    */

    let limit_x = pos.get(0).unwrap().len() as i32;
    let limit_y = pos.len() as i32;

    let pos_arr: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1)
    ];

    if !consider_full_line {
        return pos_arr.iter()
            .map(|p| (p.0 + x as i32, p.1 + y as i32))
            .filter(|p| is_valid_pos(p.0, p.1, limit_x, limit_y))
            .map(|p| pos_at(pos, p.0, p.1))
            .filter(|s| **s == SeatStatus::Occupied)
            .count() as i32;
    }

    let mut occupied_seats = 0;
    for i in pos_arr {
        let mut seat_value: Option<SeatStatus> = None;
        let mut j = 1;
        loop {
            let new_x = x as i32 + i.0 * j;
            let new_y = y as i32 + i.1 * j;
            j += 1;

            if !is_valid_pos(new_x, new_y, limit_x, limit_y) {
                break
            }

            seat_value = Some(pos_at(pos, new_x, new_y).clone());
            if !seat_value.unwrap().eq(&SeatStatus::Floor) {
                break
            }
        }

        match seat_value {
            Some(x) => {
                if x == SeatStatus::Occupied {
                    occupied_seats += 1;
                }
            },
            None => {}
        }

    }

    return occupied_seats
}