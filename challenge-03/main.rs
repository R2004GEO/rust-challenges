// Create a function that takes two numbers (2 paramaters) and return a message with "The biggest number is " + number.

fn which_is_bigger(param1: i32, param2: i32) -> i32 {
    if param1 >= param2 {
        return param1;
    } else {
        return param2;
    }
    // Quick and dirty
}

fn main() {
    let biggest_number = which_is_bigger(5, 12);
    println!("The biggest number is {}", biggest_number);
}
