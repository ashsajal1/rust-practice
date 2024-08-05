pub struct Calculator;

pub trait CalculatorTrait {
    fn new() -> Self;
    fn add(&self, numbers: Vec<u64>) -> u64; // Change method to take `&self`
    fn sub(&self, num1: u64, num2: u64) -> u64;
    fn div(&self, num1: u64, num2: u64) -> u64;
    fn mul(&self, numbers: Vec<u64>) -> u64;
}

impl CalculatorTrait for Calculator {
    fn new() -> Self {
        Calculator
    }

    fn add(&self, numbers: Vec<u64>) -> u64 {
        numbers.iter().sum()
    }

    fn sub(&self, num1: u64, num2: u64) -> u64 {
        num1 - num2
    }

    fn div(&self, num1: u64, num2: u64) -> u64 {
        num1 / num2
    }

    fn mul(&self, numbers: Vec<u64>) -> u64 {
        let mut result: u64 = 1; // Start with 1 for multiplication
        for num in numbers {
            result *= num;
        }
        result
    }
}