const LHS: std::ops::RangeInclusive<i32> = 2..=9;
const RHS: std::ops::RangeInclusive<i32> = 1..=9;

fn main() {
    LHS
        .flat_map(|lhs| RHS.map(move |rhs| (lhs, rhs)))
        .map(|(lhs, rhs)| format!("{:>2} x {:>2} = {:>2}", lhs, rhs, lhs * rhs))
        .for_each(|line| println!("{}", line));
}
