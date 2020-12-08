#[cfg(test)]
mod tests {
    use crate::dec8::{solve_puzzle, solve_puzzle_part_b};
    use std::io::Error;
    use crate::utils::get_file;

    #[test]
    fn test_example_instructions() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(5, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_puzzle_instructions() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(1749, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_part_b_example() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle_part_b(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(8, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_part_b_puzzle() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle_part_b(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(515, result.unwrap());

        Ok(())
    }
}