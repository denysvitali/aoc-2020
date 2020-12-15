mod test {
    use crate::utils::get_file;
    use crate::solve_puzzle;

    #[test]
    fn example(){
        let f = get_file(file!(), "1.txt");
        assert_eq!(436, solve_puzzle(f.as_str(), 2020).unwrap());
    }

    #[test]
    fn puzzle(){
        let f = get_file(file!(), "2.txt");
        assert_eq!(436, solve_puzzle(f.as_str(), 2020).unwrap());
    }

    #[test]
    fn puzzle_part_2(){
        let f = get_file(file!(), "2.txt");
        assert_eq!(548531, solve_puzzle(f.as_str(), 30000000).unwrap());
    }
}