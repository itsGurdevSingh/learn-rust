use std::ops::{ Add, Div, Mul, Sub};

fn main() {
    println!("largest num is {}", find_largest(2,3));

    let result = calculate(2, "+", 3);
    match result {
        Some(sum) => println!("addition of the 2 and 3 {}", sum),
        None => {}
    }

    
}

fn find_largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}

fn calculate<T: Add + Sub + Mul + Div>(num1:T, operator: &str, num2: T) -> Option<T>
 where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
 {
    match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => Some(num1 / num2),
        _ => None
    }
}
