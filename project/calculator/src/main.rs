fn main() {
    println!("Hello, world!");

    let add = calculate(50, "*", 10);

    match add {
        Some(result) => println!("{}", result),
        None => println!("result not found"),
    }

}


fn calculate(num1: i32 , operator: &str, num2: i32) -> Option<i32> {
    match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => Some(num1 / num2),
        &_ => None,
    }
}
