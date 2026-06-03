use std::ops::{Add, Div, Mul, Sub};

fn main() {
    println!("largest num is {}", find_largest(2, 3));

    let num1 = 2;
    let num2 = 3;
    let operator = "+";

    let result = calculate(&num1, "+", &num2);
    match result {
        Some(sum) => println!("Result of {} {} {} = {}", num1, operator, num2, sum),
        None => {}
    }
}

fn find_largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}

fn calculate<T: Add + Sub + Mul + Div + Copy>(num1: &T, operator: &str, num2: &T) -> Option<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    match operator {
        "+" => Some(*num1 + *num2),
        "-" => Some(*num1 - *num2),
        "*" => Some(*num1 * *num2),
        "/" => Some(*num1 / *num2),
        _ => None,
    }
}
