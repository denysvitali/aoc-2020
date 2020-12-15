#[cfg(test)]
mod tests {
    use crate::{solve_puzzle, solve_puzzle_part_b};
    use std::io::Error;
    use utils::get_file;

    #[test]
    fn test_example() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(165, result.unwrap());
        Ok(())
    }

    #[test]
    fn test_puzzle() -> Result<(), Error>{
        let f = get_file(file!(), "3.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(10050490168421, result.unwrap());
        Ok(())
    }

    #[test]
    fn test_example_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle_part_b(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(208, result.unwrap());
        Ok(())
    }

    #[test]
    fn test_puzzle_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "3.txt");
        let result = solve_puzzle_part_b(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(2173858456958, result.unwrap());
        Ok(())
    }
}