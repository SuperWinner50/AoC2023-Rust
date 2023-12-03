fn p1() -> u32 {
    let re = regex::Regex::new("[0-9]+").unwrap();

    let all = std::fs::read_to_string("input.txt").unwrap();
    let width = all.split("\n").next().unwrap().len() + 1;

    re.find_iter(&all)
        .map(|mat| {
            let s = mat.range().start as i32;
            let (x, y) = (s % width as i32, s / width as i32);

            let w = mat.len() as i32;

            let xs = (x - 1..x + w + 1)
                .chain(std::iter::once(x + w))
                .chain(x - 1..x + w + 1)
                .chain(std::iter::once(x - 1));

            let ys = std::iter::repeat(y - 1).take(w as usize + 2)
                .chain(std::iter::once(y))
                .chain(std::iter::repeat(y + 1).take(w as usize + 2))
                .chain(std::iter::once(y));

            xs.zip(ys)            
                .filter(|&(x, y)| x >= 0 && x < width as i32 - 2 && y >= 0 && y < (all.len() / width) as i32)
                .any(|(x, y)| {
                    let i = y as usize * width + x as usize;
                    all.chars().nth(i).map_or(false, |l| l != '.' && !l.is_digit(10))
                })
                .then_some(mat.as_str().parse::<u32>().unwrap()).unwrap_or(0)
        }).sum()
}

fn p2() -> u32 {
    let re = regex::Regex::new("[0-9]+").unwrap();

    let all = std::fs::read_to_string("input.txt").unwrap();
    let width = all.split("\n").next().unwrap().len() + 1;

    re.find_iter(&all)
        .fold(std::collections::BTreeMap::<usize, Vec<u32>>::new(), |mut state, mat| {
            let s = mat.range().start as i32;
            let (x, y) = (s % width as i32, s / width as i32);

            let w = mat.len() as i32;

            let xs = (x - 1..x + w + 1)
                .chain(std::iter::once(x + w))
                .chain(x - 1..x + w + 1)
                .chain(std::iter::once(x - 1));

            let ys = std::iter::repeat(y - 1).take(w as usize + 2)
                .chain(std::iter::once(y))
                .chain(std::iter::repeat(y + 1).take(w as usize + 2))
                .chain(std::iter::once(y));

            let n = mat.as_str().parse::<u32>().unwrap();

            xs.zip(ys)            
                .filter(|&(x, y)| x >= 0 && x < width as i32 - 2 && y >= 0 && y < (all.len() / width) as i32)
                .find_map(|(x, y)| {
                    let i = y as usize * width + x as usize;
                    all.chars().nth(i).map(|l| (l == '*').then_some(i)).flatten()
                })
                .map(|i| state.entry(i).or_default().push(n));

            state
        })
        .values()
        .filter_map(|ns| (ns.len() == 2).then(|| ns[0] * ns[1]))
        .sum()
}

fn main() {
    println!("{}", p2());
}
