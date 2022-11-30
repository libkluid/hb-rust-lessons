pub enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Num(usize),
}

impl ToString for FizzBuzz {
    fn to_string(&self) -> String {
        match self {
            FizzBuzz::Fizz => String::from("Fizz"),
            FizzBuzz::Buzz => String::from("Buzz"),
            FizzBuzz::FizzBuzz => String::from("FizzBuzz"),
            FizzBuzz::Num(value) => format!("{}", value),
        }
    }
}

fn is_multiple<const S: usize>(value: usize) -> bool {
    value % S == 0
}

impl From<usize> for FizzBuzz {
    fn from(value: usize) -> Self {
        let fizz = is_multiple::<5>(value);
        let buzz = is_multiple::<3>(value);
        
        match (fizz, buzz) {
            (false, false) => FizzBuzz::Num(value),
            (true, false)  => FizzBuzz::Fizz,
            (false, true)  => FizzBuzz::Buzz,
            (true, true)   => FizzBuzz::FizzBuzz,
        }
    }
}

pub struct RustyFizzBuzz {
    value: usize,
}

impl RustyFizzBuzz {
    pub fn new() -> Self {
        RustyFizzBuzz { value: 0 }
    }
}

impl Iterator for RustyFizzBuzz {
    type Item = FizzBuzz;
    
    fn next(&mut self) -> Option<Self::Item> {
        let (next_value, _) = self.value.overflowing_add(1);
        self.value = next_value;
        let result = FizzBuzz::from(next_value);
        Some(result)
    }
}
