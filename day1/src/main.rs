use common::read_input_file_for_project;
fn main() {
    let mut lines = read_input_file_for_project!();

    let mut biggest_sum = 0u64;
    let mut current_sum = 0u64;
    let mut sums: Vec<u64> = vec![];
    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            if current_sum > biggest_sum {
                biggest_sum = current_sum
            }
            sums.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<u64>().expect("Failed to parse u64");
        }
    }
    println!("Part 1: {} ", biggest_sum);
    sums.sort_by(|a, b| b.cmp(a));
    println!("Part 2: {} ", sums[0] + sums[1] + sums[2]);
}
