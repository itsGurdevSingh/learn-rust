use std::io;

enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
}

struct CalcInput {
    num1: i32,
    op: Operation,
    num2: i32,
}

impl CalcInput {
    fn calculate(&self) -> i32 {
        match self.op {
            Operation::Add => self.num1 + self.num2,
            Operation::Subtract => self.num1 - self.num2,
            Operation::Multiply => self.num1 * self.num2,
            Operation::Divide => self.num1 / self.num2,
        }
    }
}

fn main() {

    let mut s: String = get_user_input();

    let cal_input = params_form_input(&mut s);

    println!("result is : {}",cal_input.calculate());
   
}

// string input
fn get_user_input() -> String {
    println!("enter a value for calcuation \n  ( 1 + 2 )");
    let mut s = String::new();

    std::io::stdin()
        .read_line(&mut s)
        .expect("failed to read input");

    return s;
}

fn params_form_input(s: &str) -> CalcInput {

    let mut sl = s.split_whitespace();

    // validate is first elem is num

    let first_elem = sl
        .next()
        .expect("invalid input")
        .parse::<i32>()
        .expect(" invalid input");
    let second_elem = sl
        .next()
        .expect("invalid input");

    let third_elem = sl
        .next()
        .expect("invalid input")
        .parse::<i32>()
        .expect(" invalid input");

    let op: Operation;

    match second_elem {
        "+" => op = Operation::Add,
        "-" => op = Operation::Subtract,
        "*" => op = Operation::Multiply,
        "/" => op = Operation::Divide,
        _ => panic!("invalid Operation"),
    }

    return CalcInput {
        num1: first_elem,
        op,
        num2: third_elem,
    };
}
