#[cfg(test)]
mod tests {
    use crate::dec7::{solve_puzzle, parse_bag_line, Bag, solve_puzzle_part_b};
    use std::io::Error;
    use crate::utils::get_file;

    #[test]
    fn test_parse_bag() {
        let bag_str = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";
        let bag = parse_bag_line(bag_str.to_owned());

        assert_eq!(Bag{
            name: "muted yellow".to_string(),
            contains: vec![
                (Bag{
                    name: "shiny gold".to_string(),
                    contains: vec![]
                }, 2),
                (Bag{
                    name: "faded blue".to_string(),
                    contains: vec![]
                }, 9)
            ]
        }, bag)
    }

    #[test]
    fn test_example_bags() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(4, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_real_bags() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(4, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_part_b_example() -> Result<(), Error>{
        let f = get_file(file!(), "3.txt");
        let result = solve_puzzle_part_b(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(126, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_part_b_puzzle() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle_part_b(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(34988, result.unwrap());

        Ok(())
    }
}