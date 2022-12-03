use std::collections::HashSet;

pub fn run() -> color_eyre::Result<()> {
    let data = std::fs::read_to_string("inputs/day3.txt")?;

    let mut common_items = Vec::new();
    for line in data.lines() {
        let bytes = line.as_bytes();
        let (a, b) = bytes.split_at(bytes.len() / 2);
        let set_a: HashSet<u8> = HashSet::from_iter(a.iter().copied());
        let set_b: HashSet<u8> = HashSet::from_iter(b.iter().copied());
        let common = set_a.intersection(&set_b).next().unwrap();
        common_items.push(*common);
    }

    let sum = common_items.into_iter().map(priority).sum::<u64>();
    println!("Sum of priorities: {sum}");

    let mut badges = Vec::new();
    let lines = data.lines().collect::<Vec<_>>();

    for group in lines.chunks_exact(3) {
        let set_a: HashSet<u8> = HashSet::from_iter(group[0].as_bytes().iter().copied());
        let set_b = HashSet::from_iter(group[1].as_bytes().iter().copied());
        let set_c = HashSet::from_iter(group[2].as_bytes().iter().copied());

        let intersection_a_b = set_a.intersection(&set_b).copied().collect::<HashSet<_>>();
        let intersection = intersection_a_b.intersection(&set_c).copied().next().unwrap();

        badges.push(intersection);
    }
    let sum = badges.into_iter().map(priority).sum::<u64>();
    println!("Sum of badges priorities: {sum}");

    Ok(())
}

fn priority(item: u8) -> u64 {
    (match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!("Invalid item"),
    }) as u64
}
