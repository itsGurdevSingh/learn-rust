struct User {
    first_name: String,
    last_name:String,
    age: u32
}


fn main() {
    
    let user = User {
        first_name: String::from("Gurdev"),
        last_name: String::from("Singh"),
        age: 21
    };

    println!("name of user is {}", user.first_name);
    
}
