use regex::Regex;

fn p1() -> u32 {
    let re = Regex::new("([0-9]+) ([a-z]+)").unwrap();

    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            re.captures_iter(line).all(|mat| {
                let n = mat.get(1).unwrap().as_str().parse::<u32>().unwrap();

                match mat.get(2).unwrap().as_str() {
                    "red" => n <= 12,
                    "green" => n <= 13,
                    "blue" => n <= 14,
                    _ => panic!(),
                }
            }).then_some(i as u32 + 1)  
        })
        .sum()
}

fn p2() -> u32 {
    let re = Regex::new("([0-9]+) ([a-z]+)").unwrap();

    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            re.captures_iter(line).fold([0, 0, 0], |mut state, mat| {
                let n = mat.get(1).unwrap().as_str().parse::<u32>().unwrap();

                let i = match mat.get(2).unwrap().as_str() {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!()
                };
                
                state[i] = std::cmp::max(state[i], n);
                state
            }).iter().product::<u32>()
        })
        .sum()
}

fn main() {
    println!("{}", p1());
}
