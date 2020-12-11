#[cfg(test)]
mod tests {
    use crate::dec11::{solve_puzzle, solve_part_b};
    use std::io::Error;
    use crate::utils::get_file;

    #[test]
    fn test_example() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str(), 4);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(37, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_puzzle() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle(f.as_str(), 4);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(37, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_example_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str(), 5);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(26, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_puzzle_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle(f.as_str(), 5);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(1862, result.unwrap());

        Ok(())
    }
}