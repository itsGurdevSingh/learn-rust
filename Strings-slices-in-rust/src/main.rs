fn main() {
    // STRING LITERALS 
    // this si string literal type is &str it is harcorded strings in the program.
    let str_litral = "gurdev singh";

    // dynamic string
    let str = String::from("String type string");

    // window to String specific slice or portion. 
    let and_str = &str[7..11];

    //both has sae type &str
    println!(" string literals{}", str_litral);
    println!(" window to String &str{}", and_str);

    println!(" first number in string {} \n {}",str , find_first_word(&str));

    
}

fn find_first_word(str: &String) -> &str {

    let mut index = 0;

    for char in str.chars(){
        if char == ' ' {
            break ;
        }
        index = index + 1;
    }
    return &str[0..index];
}
