use std::io;
use std::io::BufRead;
use stoogesort::Stooge;
fn main() {
    if atty::is(atty::Stream::Stdin) {
        println!("Pipe in a newline-separated list of ints");
        return;
    }

    let mut nums: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
    nums.stooge_sort();

    for n in nums {
        println!("{}", n);
    }
}
