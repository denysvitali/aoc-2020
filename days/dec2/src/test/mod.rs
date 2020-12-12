#[cfg(test)]
mod tests {
    use crate::{solve_puzzle, get_passwords, PasswordPolicy, solve_puzzle_2};
    use std::io::Error;
    use utils::get_file;

    #[test]
    fn test_get_passwords() -> Result<(), Error> {
        let passwords = get_passwords(&get_file(file!(), "1.txt"));
        assert_eq!(vec!(
            (PasswordPolicy{
                min_rep: 1,
                max_rep: 3,
                char: 'a'
            }, "abcde".to_owned()),
            (PasswordPolicy{
                min_rep: 1,
                max_rep: 3,
                char: 'b'
            }, "cdefg".to_owned()),
            (PasswordPolicy{
                min_rep: 2,
                max_rep: 9,
                char: 'c'
            }, "ccccccccc".to_owned()),
        ), passwords?);
        Ok(())
    }

    #[test]
    fn test_example_passwords() -> Result<(), Error>{
        let f = get_file(file!(), "1.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(2, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_real_passwords() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(456, result.unwrap());

        Ok(())
    }

    #[test]
    fn test_real_passwords_2() -> Result<(), Error>{
        let f = get_file(file!(), "2.txt");
        let result = solve_puzzle_2(f.as_str());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        assert_eq!(308, result.unwrap());

        Ok(())
    }
}