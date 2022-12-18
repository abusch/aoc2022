use std::collections::HashSet;

use eyre::Result;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day06.txt")?;

    let bytes = data.as_bytes();
    let index = start_of_packet(bytes);
    println!("Part 1: start-of-packet at index: {index}");

    let index = start_of_message(bytes);
    println!("Part 2: start-of-message at index: {index}");

    Ok(())
}

fn start_of_packet(data: &[u8]) -> usize {
    find_distinct_sequence(data, 4)
}

fn start_of_message(data: &[u8]) -> usize {
    find_distinct_sequence(data, 14)
}

fn find_distinct_sequence(data: &[u8], len: usize) -> usize {
    let index = data
        .windows(len)
        .position(|window| {
            let set = window.iter().copied().collect::<HashSet<_>>();
            set.len() == len
        })
        .unwrap();
    index + len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_of_packet() {
        let data = b"bvwbjplbgvbhsrlpgdmjqwftvncz";
        let index = start_of_packet(data);
        assert_eq!(index, 5);

        let data = b"nppdvjthqldpwncqszvftbrmjlhg";
        let index = start_of_packet(data);
        assert_eq!(index, 6);

        let data = b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let index = start_of_packet(data);
        assert_eq!(index, 10);

        let data = b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let index = start_of_packet(data);
        assert_eq!(index, 11);
    }
}
