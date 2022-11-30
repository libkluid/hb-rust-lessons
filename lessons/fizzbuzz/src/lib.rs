mod fizzbuzz;
use fizzbuzz::RustyFizzBuzz;

pub fn fizzbuzz(num: usize) {
    let fizzbuzz = RustyFizzBuzz::new();
    for value in fizzbuzz.take(num) {
        println!("{}", value.to_string());
    }
}
