fn main() {
    // ownership one variable has only one owner at time
    // moving previous owner is discard on moing ownership
    // borrowing and refrences pass pointer of variable owner stay same .

    let s1 = String::from("gurdev");
    let s2 = s1;

    //value of s1 is moved to s2 so we will not use s1 any more .
    // println!("value of s1 is {}", s1);
    println!("value of s2 is {}", s2);


    // clone make duplicate copy in heap now we have totaly saperate variables in heap and both have spaerate owners
    let s3 = s2.clone();
    println!("value of s2 is {}", s2);
    println!("value of s3 is {}", s3);


    // we have borrwed the variable ownership still belong to original variable .
    let s4 = &s3;
    println!("value of s4 is {}", s4);
}
