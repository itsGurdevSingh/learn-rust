fn main() {
    let mut vec = vec![1, 2, 3, 4];

    let vec_clone = vec.clone();

    let iter = vec.iter();

    for val in iter {
        println!("val is {}", *val);
    }

    // above for loop for ownership of that iter so that was no more outside the scope of for loop.
    // let original_iter = iter;

    // but we can still use vec because it has not pass his ownership to iter.
    println!("orignal vec is after .iter() {:?}", vec);

    let iter_mut = vec.iter_mut();

    for val in iter_mut {
        *val = *val + 1;
    }

    println!("value of vec after mutation via .iter_mut is {:?}", vec);

    let iter_into = vec.into_iter();

    for val in iter_into {
        let val_copy = val - 1;
        // val = vall;
        print_val(val_copy);
    }

    // we are not able to use vec further because we iter_into accept ownerhip.
    // println!("value of vec after iter into is {:?}", vec);

    let sum: i32 = vec_clone.iter().sum();

    println!("sum of all elems in cloned vec is : {}", sum);

    let even_num: Vec<i32> = vec_clone
        .iter()
        .filter(|x| **x % 2 == 0)
        .map(|x| x * 2)
        .collect();

    println!(
        "cloned vec first filter even number and then double then {:?}",
        even_num
    );

    println!("clone of vec is :{:?}", vec_clone);

    println!("\n\niter via while loop :-");
    iter_while_loop();
}

fn print_val(num: i32) {
    println!("val is {}", num);
}

fn iter_while_loop() {
    let mut v1 = vec![1, 2, 3];

    let mut iter = v1.iter_mut();

    while let Some(val) = iter.next() {
        println!("val in while is {}" , val);
    }
}
