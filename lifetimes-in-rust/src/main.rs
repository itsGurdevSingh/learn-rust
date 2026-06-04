struct User<'a, 'b> {
    name: &'a str,
    age: &'b u32,
}

fn main() {
    let largest_str: &str;
    let str2: String = String::from("largest");

    let str1 = String::from("small");
    {
        largest_str = largest(&str1, &str2);
    }
    println!("{}", largest_str);

    let user: User;

    let name = "gurdev";

    // if we put that age in scope so it it will give error for for lifetime
    //  because age is refrence and it will distroied after this scope.
    // so later on we are not able to use this age in users also 
    // so our rust comiler though error to protect form dangling pointer.
    // {
    let age = 22;
        user = User {
            name: name,
            age: &age,
        };
        
    // }
        println!("{} is {} year old.", user.name, user.age);
}

fn largest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    }
    return str2;
}
