struct User {
    first_name: String,
    last_name:String,
    age: u32
}


struct Rect {
    width: u32,
    height: u32,
}

impl Rect{

    fn area(&self) -> u32{
        self.width * self.height
    }

    fn parimeter(&self) -> u32 {
        2*(self.width + self.height)
    }

    // like a static function in js classes 
    fn debug() -> u32 {
        return 1
    }
}

fn main() {
    
    let user = User {
        first_name: String::from("Gurdev"),
        last_name: String::from("Singh"),
        age: 21
    };

    println!("name of user is {}", user.first_name);
    
    
    let rect1 = Rect{
        width: 20,
        height: 10,
    };

    println!("area of rect1 is {}" , rect1.area());
    println!("parimeter of rect1 is {}", rect1.parimeter());


    // static function or fuction withour self refrence are called on struct itself on in its objects 
    println!("run debug fn on Rect return value is {}", Rect::debug())
 
}
