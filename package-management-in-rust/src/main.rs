use chrono::{Local, Utc };

fn main() {

    let utc_now = Utc::now();

    let current_time = Local::now();

    println!("current Utc time is {} \n current LOcal time is {}" , utc_now, current_time);


    let formatted = current_time.format("%d-%m-%y");

    println!("formated local date is {}", formatted);


}
