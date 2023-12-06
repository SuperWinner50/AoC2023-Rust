static INPUT: &str = include_str!("input.txt");

fn p1() {
    let mut it = INPUT.split(":").skip(1);
    let seeds = it.next().unwrap().trim_matches(|c: char| !c.is_ascii_digit()).split(" ").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let maps: Vec<Vec<_>> = it.map(|s| 
        s.trim_matches(|c: char| !c.is_ascii_digit()).lines().map(|l| {
            let mut line_it = l.split(" ");
            [1, 2, 3].map(|_| line_it.next().unwrap().parse::<u64>().unwrap())
        }).collect::<Vec<_>>()
    ).collect();

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
    let mut it = INPUT.split(":").skip(1);
    let seeds = it.next().unwrap().trim_matches(|c: char| !c.is_ascii_digit()).split(" ").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let seeds = seeds.chunks_exact(2).map(|c| [c[0], c[1]]).collect::<Vec<_>>();

    let x = it.fold(seeds, |v_ranges, s| {
        let mut map = s.trim_matches(|c: char| !c.is_ascii_digit()).lines().map(|l| {
            let mut line_it = l.split(" ");
            [1, 2, 3].map(|_| line_it.next().unwrap().parse::<u64>().unwrap())
        }).collect::<Vec<_>>();

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

        new_ranges
    }).into_iter().map(|r| r[0]).min().unwrap();

    println!("{x}");
}

fn main() {
    p1();
}
