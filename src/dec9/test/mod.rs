#[cfg(test)]
mod tests {
    use crate::dec9::{solve_puzzle, solve_part_b};
    use std::io::Error;
    use crate::utils::get_file;

    #[test]
    fn test_example_xmas() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str(), 5);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(127, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_puzzle_xmas() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle(f.as_str(), 25);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(27911108, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_example_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_part_b(f.as_str(), 5);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(62, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_puzzle_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_part_b(f.as_str(), 25);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(4023754, result.unwrap());

        Ok(())
    }
}