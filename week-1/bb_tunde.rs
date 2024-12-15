use std::io;

fn main() {
    const MAX_STAFF: usize = 100;

    // Iterate for each staff member (up to MAX_STAFF)
    for i in 0..MAX_STAFF {
        // Ask for the name of the staff member
        println!("Enter the name of staff member #{}:", i + 1);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim(); // Remove any leading/trailing whitespace

        // Ask for the number of papers published
        println!("Enter the number of papers published by {}:", name);
        let mut papers_str = String::new();
        io::stdin().read_line(&mut papers_str).expect("Failed to read line");
        
        // Parse the input to an integer
        let papers: u32 = papers_str.trim().parse().expect("Please enter a valid number");

        // Calculate the incentive based on the number of papers published
        let incentive = calculate_incentive(papers);

        // Output the name and the incentive for this staff member
        println!("Staff Member: {}, Incentive: N{}", name, incentive);
        println!("-----------------------------------------");
    }
}

// Function to calculate the incentive based on the number of papers published
fn calculate_incentive(papers: u32) -> u32 {
    if papers < 3 {
        100_000 // N100,000 for less than 3 papers
    } else if papers >= 3 && papers <= 5 {
        500_000 // N500,000 for 3 to 5 papers
    } else if papers > 5 && papers < 10 {
        800_000 // N800,000 for more than 5 but less than 10 papers
    } else {
        1_000_000 // N1,000,000 for 10 or more papers
    }
}
