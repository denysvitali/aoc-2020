#[cfg(test)]
mod tests {
    use crate::{solve_puzzle, solve_puzzle_part_b, xgcd};
    use std::io::Error;
    use utils::get_file;

    #[test]
    fn test_xgcd(){
        assert_eq!((2, 0, 1), xgcd(10, 2));
        assert_eq!((1, 1, -18712), xgcd(243257,13));
    }

    #[test]
    fn test_example() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(295, result.unwrap());
        Ok(())
    }

    #[test]
    fn test_puzzle() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(2095, result.unwrap());
        Ok(())
    }

    #[test]
    fn test_example_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle_part_b(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(1068781, result.unwrap());
        Ok(())
    }

    #[test]
    fn test_puzzle_part_b() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle_part_b(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(598411311431841, result.unwrap());
        Ok(())
    }
}