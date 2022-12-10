use common::read_input_file_for_project;
fn main() {
    let mut lines = read_input_file_for_project!();

    // Variables to store results
    let mut biggest_sum = 0u64;
    let mut current_sum = 0u64;
    let mut sums: Vec<u64> = vec![];
    while let Some(Ok(line)) = lines.next() {
        // If the line is empty, we finished this entry so save the values
        if line.is_empty() {
            if current_sum > biggest_sum {
                biggest_sum = current_sum
            }
            // For part 2 just save all the sums
            sums.push(current_sum);
            current_sum = 0;
        } else {
            // Otherwise add to the current sum
            current_sum += line.parse::<u64>().expect("Failed to parse u64");
        }
    }
    // Part 1 just needs the biggest sum
    println!("Part 1: {} ", biggest_sum);
    // Part 2 needs the 3 biggest sums, so sort the sums in descending order and print.
    sums.sort_by(|a, b| b.cmp(a));
    println!("Part 2: {} ", sums[0] + sums[1] + sums[2]);
}
