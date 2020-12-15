#[macro_use]
extern crate bencher;
use bencher::Bencher;

extern crate dec14;
use dec14::{solve_puzzle, solve_puzzle_part_b};
use std::path::Path;

fn bench_part_b(b: &mut Bencher){
    b.iter(||{
        let f = Path::new(file!())
            .parent().unwrap().join(format!("{}{}", "../src/test/input/", "2.txt"));
        let result = solve_puzzle_part_b(f.to_str().unwrap()).unwrap();
    })
}

benchmark_group!(benches, bench_part_b);
benchmark_main!(benches);