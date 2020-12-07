use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

mod test;

#[derive(Debug, Clone)]
struct Bag {
    pub name: String,
    pub contains: Vec<(Bag, i32)>
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false
        }

        return true
    }
}

impl Eq for Bag {

}

/*
    Returns the number of bags that can contain a shiny gold bag
 */
fn solve_puzzle(path: &str) -> io::Result<i32> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut parents_for_bag: HashMap<Bag, Vec<Bag>> = HashMap::new();
    for line in reader.lines(){
        let bag = parse_bag_line(line.unwrap());
        for inner_bag in &bag.contains {
            let bag_ref = &inner_bag.0;
            if !parents_for_bag.contains_key(&inner_bag.0) {
                parents_for_bag.insert(bag_ref.clone(), Vec::new());
            }

            let vec = parents_for_bag.get_mut(&bag_ref).unwrap();
            vec.push(
                bag.clone()
            )
        }
    }

    let needle = Bag{
        name: String::from("shiny gold"),
        contains: vec![]
    };

    for (k, v) in parents_for_bag.borrow() {
        println!("{} contains {:?}", k.name, format_bag_vec(v));
    }

    let mut hs = HashSet::new();
    let result = count_parents(&mut hs, &needle, parents_for_bag.borrow());

    println!("Outer colors: {:?}", parents_for_bag);

    Ok(result as i32)
}

fn solve_puzzle_part_b(path: &str) -> io::Result<i32> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut children_of_bag: HashMap<Bag, Vec<(Bag, i32)>> = HashMap::new();
    for line in reader.lines(){
        let bag = parse_bag_line(line.unwrap());
        for inner_bag in &bag.contains {
            if children_of_bag.get(&bag).is_none() {
                children_of_bag.insert(bag.clone(), Vec::new());
            }

            children_of_bag.get_mut(&bag).unwrap().push(
                inner_bag.clone()
            )
        }
    }

    let needle = Bag{
        name: String::from("shiny gold"),
        contains: vec![]
    };

    let result = count_parents_2(1, &needle, children_of_bag.borrow());


    Ok(result as i32)
}

fn format_bag_vec(p0: &Vec<Bag>) -> String {
    p0.iter().map(|x| x.name.to_owned()).collect::<Vec<_>>().join(", ")
}

fn count_parents(outer_colors: &mut HashSet<String>, ref_bag: &Bag, parents_for_bag: &HashMap<Bag, Vec<Bag>>) -> usize {
    if !parents_for_bag.contains_key(&ref_bag){
        outer_colors.insert(ref_bag.name.clone());
        return outer_colors.len();
    }

    let bags = parents_for_bag.get(ref_bag).unwrap();
    for bag in bags {
        outer_colors.insert(bag.name.clone());
        count_parents(outer_colors,bag, parents_for_bag.borrow());
    }
    return outer_colors.len();
}

fn count_parents_2(parent_count: i32, ref_bag: &Bag, children_of_bag: &HashMap<Bag, Vec<(Bag, i32)>>) -> i32 {
    let mut count = 0;
    if children_of_bag.get(ref_bag).is_none() {
        return count;
    }
    let bags = children_of_bag.get(ref_bag).unwrap();
    for bag in bags {
        count += parent_count * bag.1;
        count += count_parents_2(parent_count * bag.1, &bag.0, children_of_bag.borrow());
    }
    return count;
}

fn parse_bag_line(line: String) -> Bag {
    let re = Regex::new(r"^(.*?) bags contain (.*)\.$").unwrap();
    let captures = re.captures(&line).unwrap();

    if captures[2] == "no other bags".to_owned() {
        return Bag{ name: captures[1].to_string(), contains: vec![] }
    }

    let contains = parse_bag_contains(captures[2].to_string());
    return Bag{ name: captures[1].to_string(), contains }
}

fn parse_bag_contains(contains: String) -> Vec<(Bag, i32)>{
    let mut bags : Vec<(Bag, i32)> = Vec::new();
    let split = contains.split(", ");
    let re = Regex::new(r"^(\d+) (.*?) bag(s|)").unwrap();

    for i in split {
        let captures = re.captures(i).unwrap();
        let amount_bags = captures[1].parse::<i32>().unwrap();
        bags.push((Bag{ name: captures[2].to_owned(), contains: vec![] }, amount_bags));
    }

    return bags;
}