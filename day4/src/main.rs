static INPUT: &str = include_str!("input.txt");

fn p1() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .fold((false, vec![], 0), |(check, mut winning, mut score), n| {
                    if n == "|" {
                        return (true, winning, score);
                    }

                    if !check {
                        winning.push(n);
                    } else {
                        if winning.contains(&n) {
                            if score == 0 {
                                score = 1;
                            } else {
                                score *= 2;
                            }
                        }
                    }

                    (check, winning, score)
                }).2
        })
        .sum()
}

fn p2() -> u32 {
    INPUT
        .lines()
        .enumerate()
        .fold(std::collections::BTreeMap::new(), |mut copies, (i, line)| {
            let new = line.split_whitespace()
                .skip(2)
                .fold((false, vec![], 0), |(check, mut winning, mut score), n| {
                    if n == "|" {
                        return (true, winning, score);
                    }

                    if !check {
                        winning.push(n);
                    } else {
                        if winning.contains(&n) {
                            score += 1;
                        }
                    }

                    (check, winning, score)
                }).2 as usize;

            let amount = *copies.entry(i).and_modify(|c| *c += 1).or_insert(1);

            for k in i + 1..=i + new {
                copies.entry(k).and_modify(|c| *c += amount as u32).or_insert(amount as u32);
            }

            copies
        })
        .values()
        .copied()
        .sum()
}

fn main() {
    println!("{}", p2());
}
