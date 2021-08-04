// Take an array of numbers and print the total

fn main() {
    let super_array = [5, 10, 88, 77, 2871];
    let array_total = super_array.len();
    let mut total = 0;
    for i in 0..array_total {
        total += super_array[i]
    }
    println!("{}", total);
}
