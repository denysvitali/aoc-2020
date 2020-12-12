#[cfg(test)]
mod tests {
    use crate::dec12::{solve_puzzle};
    use std::io::Error;
    use crate::utils::get_file;

    #[test]
    fn test_example() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str(), (0,0), false);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(25, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_puzzle() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle(f.as_str(), (0,0), false);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(1496, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_example_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str(), (10,1), true);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(286, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_puzzle_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle(f.as_str(), (10,1), true);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(63843, result.unwrap());

        Ok(())
    }
}