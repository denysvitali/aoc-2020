mod test {
    use crate::utils::get_file;
    use crate::{solve_puzzle, solve_puzzle_part_2};

    #[test]
    fn example(){
        let f = get_file(file!(), "1.txt");
        assert_eq!(71, solve_puzzle(f.as_str()).unwrap());
    }

    #[test]
    fn puzzle(){
        let f = get_file(file!(), "2.txt");
        assert_eq!(25972, solve_puzzle(f.as_str()).unwrap());
    }

    #[test]
    fn example_part_2(){
        let f = get_file(file!(), "3.txt");
        assert_eq!(1, solve_puzzle_part_2(f.as_str()).unwrap());
    }

    #[test]
    fn puzzle_part_2(){
        let f = get_file(file!(), "2.txt");
        assert_eq!(622670335901, solve_puzzle_part_2(f.as_str()).unwrap());
    }

}