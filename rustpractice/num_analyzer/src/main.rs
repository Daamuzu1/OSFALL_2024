// Function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Creating an array of 10 random integer numbers
    let numbers = [12, 15, 20, 7, 9, 18, 22, 30, 35, 40];

    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            println!("{}: {}", num, if is_even(num) { "even" } else { "odd" });
        }
    }
    // Find the sum of all numbers in the array using a while loop
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of numbers: {}", sum);

    // Find the largest number in the array
    let mut max_num = numbers[0];
    for &num in &numbers {
        if num > max_num {
            max_num = num;
        }
    }
    println!("Largest number: {}", max_num);
}
