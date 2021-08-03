// Create a function that takes a number and returns a string that returns "Even" if the number is "even" or "Odd" if the number is "odd"

fn odd_or_even(number: i32) -> String {
    if number % 2 == 0 {
        return "Even".to_string();
    } else {
        return "Odd".to_string();
    }
}

fn main() {
    let my_number: i32 = 10;
    println!("Nunmber {} is: {}", my_number, odd_or_even(my_number))
}
