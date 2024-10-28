use std::io::{self, Write};

fn main() {
    // Prompt the user for the original salary
    print!("Enter your original salary: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let original_salary: f64 = input.trim().parse().expect("Invalid input");

    // Prompt the user for the new salary after decrease
    print!("Enter your new decreased salary: ");
    io::stdout().flush().expect("Failed to flush stdout");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let new_salary: f64 = input.trim().parse().expect("Invalid input");

    // Calculate the percentage decrease
    let percentage_decrease = ((original_salary - new_salary) / original_salary) * 100.0;

    // Print the result
    println!("Original Salary: {:.2}", original_salary);
    println!("New Decreased Salary: {:.2}", new_salary);
    println!("Percentage Decrease: {:.2}%", percentage_decrease);
}
