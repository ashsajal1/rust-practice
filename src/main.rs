pub mod calculator;
use calculator::{Calculator, CalculatorTrait};

fn main() {
    let calc = Calculator::new();
    let sum = calc.add(vec![1,2,3]);
    println!("{}", sum)
}
