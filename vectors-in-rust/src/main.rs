fn main() {
    let mut vec: Vec<i32> = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    vec.pop();
    
    vec.push(4);

    println!("vlaues in vector are {:?}", vec);

    println!("even numbers in vector {:?}", filter_even_values(&vec));
   
}

fn filter_even_values(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }

    return new_vec;
}
