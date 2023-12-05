static INPUT: &str = include_str!("input.txt");

fn p1() {
    let re_seeds = regex::Regex::new(r"\d+").unwrap();
    let re_ranges = regex::Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let mut it = INPUT.split(":").skip(1);
    let seeds: Vec<_> = re_seeds.find_iter(it.next().unwrap())
        .map(|c| c.as_str().parse::<u64>().unwrap())
        .collect();
    
    let maps: Vec<Vec<_>> = it
        .map(|s| re_ranges.captures_iter(s).map(|c| [1, 2, 3].map(|i| c.get(i).unwrap().as_str().parse::<u64>().unwrap())).collect())
        .collect();

    let x = seeds.into_iter().map(|seed| {
        let mut v = seed;

        for map in maps.iter() {
            for range in map.iter() {
                if v >= range[1] && v < range[1] + range[2] {
                    v = range[0] + (v - range[1]);
                    break;
                }
            }
        }

        v
    }).min().unwrap();

    println!("{x}");
}

fn p2() {
    let re_seeds = regex::Regex::new(r"(\d+) (\d+)").unwrap();
    let re_ranges = regex::Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let mut it = INPUT.split(":").skip(1);
    let mut v_ranges: Vec<_> = re_seeds.captures_iter(it.next().unwrap())
        .map(|c| [1, 2].map(|i| c.get(i).unwrap().as_str().parse::<u64>().unwrap()))
        .collect();
    
    let maps: Vec<Vec<_>> = it
        .map(|s| re_ranges.captures_iter(s).map(|c| [1, 2, 3].map(|i| c.get(i).unwrap().as_str().parse::<u64>().unwrap())).collect())
        .collect();

    for mut map in maps {
        map.sort_by_key(|r| r[1]);

        let mut i = 0;
        let mut j = v_ranges[i][0];

        let mut new_ranges = vec![];

        while i < v_ranges.len() {
            let end_range = v_ranges[i][0] + v_ranges[i][1];

            let min_range;

            if let Some([d, s, r]) = map.iter().find(|[_, s, r]| j >= *s && j < *s + *r) {
                // Inside of mapping
                min_range = std::cmp::min(s + r, end_range);
                new_ranges.push([d + (j - s), min_range - j]);
            } else if let Some(&[_, s, _]) = map.iter().find(|[_, s, _]| *s >= j) {
                // Outside of mapping
                min_range = std::cmp::min(s, end_range);
                new_ranges.push([j, min_range - j]);
            } else {
                // No mapping
                new_ranges.push(v_ranges[i]);
                i += 1;
                continue;
            }

            if min_range == end_range {
                i += 1;
                if i < v_ranges.len() { j = v_ranges[i][0]; }
            } else {
                j = min_range;
            }
        }

        v_ranges = new_ranges;
    }

    let smallest = v_ranges.into_iter().map(|r| r[0]).min().unwrap();
    println!("{}", smallest);
}

fn main() {
    p2();
}
