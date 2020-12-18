mod test {
    use crate::{solve_puzzle, evaluate_expr};
    use utils::get_file;

    #[test]
    fn example(){
        let f = get_file(file!(), "1.txt");
        assert_eq!(26 + 437 + 12240 + 13632, solve_puzzle(f.as_str()).unwrap());
    }

    #[test]
    fn expr_0(){
        assert_eq!(51, evaluate_expr("1 + (2 * 3) + (4 * (5 + 6))"))
    }

    #[test]
    fn expr_1(){
        assert_eq!(26, evaluate_expr("2 * 3 + (4 * 5)"))
    }


}
