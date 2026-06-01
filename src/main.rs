fn main() {
    println!("{}", is_even(20));

    println!("fib value is {}", fib(5));


    // call get string length function 
    let name = String::from("gurdev");
    let len = get_str_len(name);
    println!("length of name string is {}", len)
}

// find even number
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }

    return false;
}

// find fib number in series

fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 0..(num - 1) {
        let temp = second;
        second = first + second;
        first = temp;
    }

    return second;
}


// get length of string
fn get_str_len(s: String) -> usize {
    return s.chars().count();
}

