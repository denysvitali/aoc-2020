#[cfg(test)]
mod tests {
    use crate::dec1::{solve_puzzle, get_ints, solve_puzzle_2};
    use std::io::Error;
    use crate::utils::get_file;

    #[test]
    fn test_get_ints() -> Result<(), Error> {
        let ints = get_ints(&get_file(file!(), "1.txt"));
        assert_eq!(vec!(1721, 979, 366, 299, 675, 1456), ints?);
        Ok(())
    }

    #[test]
    fn expense_report() -> Result<(), Error> {
        let puzzle_result = 514579;
        let result = solve_puzzle(&get_file(file!(), "1.txt"));
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(true, result.is_ok());
        assert_eq!(puzzle_result, result.unwrap());

        return Ok(());
    }

    #[test]
    fn expense_report_2() -> Result<(), Error> {
        let puzzle_result = 241861950;
        let result = solve_puzzle_2(&get_file(file!(), "1.txt"));
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(true, result.is_ok());
        assert_eq!(puzzle_result, result.unwrap());

        return Ok(());
    }

    #[test]
    fn real_expense_report() -> Result<(), Error> {
        let puzzle_result = 935419;
        let result = solve_puzzle(&get_file(file!(),"2.txt"));
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(true, result.is_ok());
        assert_eq!(puzzle_result, result.unwrap());

        return Ok(());
    }

    #[test]
    fn real_expense_report_2() -> Result<(), Error> {
        let puzzle_result = 49880012;
        let result = solve_puzzle_2(&get_file(file!(),"2.txt"));
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        assert_eq!(true, result.is_ok());
        assert_eq!(puzzle_result, result.unwrap());

        return Ok(());
    }
}