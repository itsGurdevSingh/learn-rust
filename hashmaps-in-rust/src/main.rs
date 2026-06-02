use std::collections::HashMap;

fn main() {
    let mut users: HashMap<i32, String> = HashMap::new();

    users.insert(1, String::from("gurdev"));
    users.insert(2, String::from("vinay"));

    println!("users in hashmap are {:?}", users);

    match users.get(&1) {
        Some(user) => println!("user with id 1 is {}", user),
        None => println!("user with key 1 not exist in hashmap"),
    }

    match users.remove(&2) {
        Some(user) => println!("user {} is removed from hashmap", user),
        None => println!("user with key 2 not exist in hashmap to remove"),
    }

    println!("users in hashmap are {:?}", users);



    let mut users_vec: Vec<(i32,String)> = Vec::new();

    users_vec.push((1,String::from("gurdev")));
    users_vec.push((2,String::from("vinay")));

    println!("map created form vec of tuples is {:?}", create_map(users_vec));


}

fn create_map(vec: Vec<(i32, String)>) -> HashMap<i32, String> {

    let mut new_hashmap: HashMap<i32, String> = HashMap::new();

    for (key, value) in vec {
        new_hashmap.insert(key,value);
    }

    return new_hashmap;
}

