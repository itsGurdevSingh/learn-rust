use ::std::fs::read_to_string;

enum Shape {
    Rectange(f64, f64),
    Circle(f64),
    Square(f64),
}

fn main() {
    let my_shape = Shape::Rectange(20.0, 10.0);

    let area_of_my_shape = calculate_area(my_shape);

    println!("area of shape is {}", area_of_my_shape);

    // option enum usefull for error handling
    // we not have null like value in rust their we have option in which we have some or none
    let name = String::from("Gurdev Singh");

    let index_of_a = find_first_a(name);

    match index_of_a {
        Some(idx) => println!("in given string first char a exist on {} index", idx),
        None => println!("in give string thier is no char \"a\" exist"),
    }

    // result enum
    let result = read_to_string("a.txt");

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("error occures in file read operation \n {}", err),
    }
}

fn find_first_a(s: String) -> Option<u32> {
    for (idx, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(idx as u32);
        }
    }

    return None;
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectange(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(side) => side * side,
    }
}
