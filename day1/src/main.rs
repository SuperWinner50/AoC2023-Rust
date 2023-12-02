fn p1() -> u32 {
    std::fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| {
            let nums = line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();

            nums[0] * 10 + nums[nums.len() - 1]
        })
        .sum()
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

fn p2() -> u32 {
    std::fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| {
            let nums = (0..line.len()).filter_map(|i|
                line[i..i + 1].parse::<u32>().ok().or(
                    NUMS.iter()
                        .enumerate()
                        .find(|(_, &num)| line[i..].starts_with(num))
                        .map(|n| n.0 as u32 + 1)
                )
            ).collect::<Vec<_>>();

            nums[0] * 10 + nums[nums.len() - 1]
        })
        .sum()
}

fn p2_better() -> u32 {
    std::fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| {
            let nums = (0..line.len()).filter_map(|i|
                line[i..i + 1].parse::<u32>().ok().or(
                    NUMS.iter()
                        .enumerate()
                        .find(|(_, &num)| line[i..].starts_with(num))
                        .map(|n| n.0 as u32 + 1)
                )
            ).collect::<Vec<_>>();

            nums[0] * 10 + nums[nums.len() - 1]
        })
        .sum()
}

fn main() {
    println!("{}", p2());
}
