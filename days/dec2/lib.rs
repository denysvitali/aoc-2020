use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use regex::Regex;

mod test;

/*
    Returns the amount of passwords that do not respect
    the Official Toboggan Corporate Policy
 */
pub(crate) fn solve_puzzle(path: &str) -> io::Result<i32> {
    // Parse each line and get an array of ints
    let passwords = get_passwords(path)?;

    let mut valid_passwords = 0;

    for p in passwords {
        let result = is_valid_password(&p);
        if result {
            valid_passwords+=1;
        }
    }

    Ok(valid_passwords)
}

/*
    Returns the amount of passwords that do not respect
    the REAL Official Toboggan Corporate Policy
 */
fn solve_puzzle_2(path: &str) -> io::Result<i32> {
    // Parse each line and get an array of ints
    let passwords = get_passwords(path)?;

    let mut valid_passwords = 0;

    for p in passwords {
        let result = is_valid_password_2(&p);
        if result {
            valid_passwords+=1;
        }
    }

    Ok(valid_passwords)
}

fn is_valid_password(p: &(PasswordPolicy, String)) -> bool {
    let mut amount_char = 0;
    for c in p.1.chars() {
        if c == p.0.char {
            amount_char += 1;
        }
    }

    amount_char >= p.0.min_rep && amount_char <= p.0.max_rep
}

fn is_valid_password_2(p: &(PasswordPolicy, String)) -> bool {
    let mut found_chars = 0;
    let chars : Vec<char> = p.1.chars().collect();

    let i = (p.0.min_rep - 1) as usize;
    let j = (p.0.max_rep - 1) as usize;

    if chars[i] == p.0.char {
        found_chars += 1;
    }

    if chars[j] == p.0.char {
        found_chars += 1;
    }

    return found_chars == 1;
}

#[derive(Debug)]
struct PasswordPolicy {
    pub min_rep: i32,
    pub max_rep: i32,
    pub char: char
}

impl PartialEq for PasswordPolicy {
    fn eq(&self, other: &Self) -> bool {
        if self.max_rep != other.max_rep {
            return false;
        }

        if self.min_rep != other.min_rep {
            return false;
        }

        if self.char != other.char {
            return false;
        }

        return true
    }
}

fn get_passwords(path: &str) -> io::Result<Vec<(PasswordPolicy, String)>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut passwords: Vec<(PasswordPolicy, String)> = vec!();
    let re = Regex::new(r"^(\d+)-(\d+) ([A-z0-9]): (.*?)$").unwrap();

    for line in reader.lines() {
        let this_line = line?.to_owned();
        let captures = re.captures(&this_line).unwrap();
        let m_chars = captures[3].chars();
        let first_char = m_chars.into_iter().next().unwrap();
        passwords.push((PasswordPolicy{
            min_rep: (captures[1]).parse().unwrap(),
            max_rep: (captures[2]).parse().unwrap(),
            char: first_char
        }, captures[4].to_owned()));
    }

    Ok(passwords)
}