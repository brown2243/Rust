fn main() {
    sum_of_minimums([[7, 9, 8, 6], [6, 5, 4, 3], [5, 7, 4, 5], [7, 9, 4, 3]]);
}

fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers
        .map(|arr| arr.into_iter().min().unwrap())
        .iter()
        .sum()
}
