fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod day4_test {
    use super::*;

    const TEST_INPUT: &[&str] = &[
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ];

    #[test]
    fn test_count_xmas() {
        assert_eq!(18, XmasCounter::count(TEST_INPUT));
    }
}
