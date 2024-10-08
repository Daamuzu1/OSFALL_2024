
fn sum_with_step(sum: &mut i32, low: i32, high: i32, step: i32) {
    // Initialize the sum to 0
    *sum = 0;

    // Iterate from low to high with the given step
    for num in (low..=high).step_by(step as usize) {
        *sum += num;
    }
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}


