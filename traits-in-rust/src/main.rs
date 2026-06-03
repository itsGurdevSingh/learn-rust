pub trait Summary {
    fn summarize(&self) -> String {
        format!("summary is not implemented")
    }
}

pub trait Fix {
    fn fix(&self) -> String {
        format!("this is fix res")
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

impl Fix for User {}

fn main() {
    let user: User = User {
        name: String::from("Gurdev"),
        age: 22,
    };

    print_user_summary(&user);

    println!("{}", user.fix());

    print_user_fix(&user);

    let user_returned = return_impl_trait();

    println!("\n\nuser return summary :- \n{}", user_returned.summarize());
}

fn print_user_summary(user: &impl Summary) {
    println!("summary :- \n{}", user.summarize());
}

fn print_user_fix<T: Summary + Fix>(user: &T) {
    println!("\n\nsummary form Generic type function :- \n{}", user.summarize());
    println!("user fix string form Fix trait {}", user.fix());
}

fn return_impl_trait() -> impl Summary {
    let user2 = User{
        name: String::from("vinay"),
        age:45
    };
    return user2;
}