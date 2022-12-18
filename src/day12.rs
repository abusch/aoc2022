use eyre::Result;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day12.txt")?;

    let bytes = data
        .lines()
        .flat_map(|line| line.as_bytes().iter().copied())
        .collect::<Vec<u8>>();
    let grid = Grid::<161, 41>::new(bytes);
    let (shortest_path, _cost) = grid.shortest_path().expect("Failed to find shortest path");
    let num_steps = shortest_path.len() - 1;
    println!("Part 1: shortest path has {num_steps} steps");

    let shortest_path_steps = grid
        .shortest_path_from_any_pos()
        .map(|v| v.0.len())
        .unwrap()
        - 1;
    println!("Part 2: shortest path from any lowest position has {shortest_path_steps}");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(usize, usize);

impl Pos {
    pub fn neighbours(&self) -> Vec<Pos> {
        [self.up(), self.down(), self.right(), self.left()]
            .iter()
            .filter_map(|v| *v)
            .collect()
    }

    pub fn up(&self) -> Option<Pos> {
        // We have to use a closure as it could underflow if eagerly evaluated
        #[allow(clippy::unnecessary_lazy_evaluations)]
        (self.1 > 0).then(|| Pos(self.0, self.1 - 1))
    }

    pub fn down(&self) -> Option<Pos> {
        Some(Pos(self.0, self.1 + 1))
    }

    pub fn left(&self) -> Option<Pos> {
        // We have to use a closure as it could underflow if eagerly evaluated
        #[allow(clippy::unnecessary_lazy_evaluations)]
        (self.0 > 0).then(|| Pos(self.0 - 1, self.1))
    }

    pub fn right(&self) -> Option<Pos> {
        Some(Pos(self.0 + 1, self.1))
    }
}

pub struct Grid<const W: usize, const H: usize> {
    data: Vec<u8>,
    start: Pos,
    goal: Pos,
}

impl<const W: usize, const H: usize> Grid<W, H> {
    pub fn new(data: Vec<u8>) -> Self {
        let start_pos = data
            .iter()
            .position(|c| *c == b'S')
            .expect("Start node not found");
        let end_pos = data
            .iter()
            .position(|c| *c == b'E')
            .expect("End node not found");

        Self {
            data,
            start: Pos(start_pos % W, start_pos / W),
            goal: Pos(end_pos % W, end_pos / W),
        }
    }

    pub fn shortest_path(&self) -> Option<(Vec<Pos>, usize)> {
        self.shortest_path_from(self.start)
    }

    pub fn shortest_path_from_any_pos(&self) -> Option<(Vec<Pos>, usize)> {
        let starting_positions = self.starting_positions();
        let mut shortest_paths = starting_positions
            .into_iter()
            .filter_map(|p| self.shortest_path_from(p))
            .collect::<Vec<_>>();

        shortest_paths.sort_by_key(|a| a.0.len());
        shortest_paths.first().cloned()
    }

    fn shortest_path_from(&self, p: Pos) -> Option<(Vec<Pos>, usize)> {
        pathfinding::prelude::astar(
            &p,
            |p| self.successors(*p),
            |p| self.distance(*p),
            |p| self.success(*p),
        )
    }

    pub fn starting_positions(&self) -> Vec<Pos> {
        (0..W * H)
            .map(|i| Pos(i % W, i / W))
            .filter(|pos| self.value(*pos) == b'a')
            .collect()
    }

    pub fn value(&self, Pos(x, y): Pos) -> u8 {
        let v = self.data[y * W + x];
        if v == b'S' {
            b'a'
        } else if v == b'E' {
            b'z'
        } else {
            v
        }
    }

    pub fn distance(&self, Pos(x, y): Pos) -> usize {
        let Pos(end_x, end_y) = self.goal;
        (x.abs_diff(end_x)) + y.abs_diff(end_y)
    }

    fn successors(&self, pos: Pos) -> Vec<(Pos, usize)> {
        let candidates = pos.neighbours();
        candidates
            .into_iter()
            .filter(|p| p.0 < W && p.1 < H && self.value(*p) <= self.value(pos) + 1)
            .map(|p| (p, 1))
            .collect()
    }

    fn success(&self, pos: Pos) -> bool {
        pos == self.goal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = b"SabqponmabcryxxlaccszExkacctuvwjabdefghi".to_vec();
        let grid = Grid::<8, 5>::new(data);
        let (shortest_path, _cost) = grid.shortest_path().unwrap();

        assert_eq!(31, shortest_path.len() - 1)
    }

    #[test]
    fn test_part2() {
        let data = b"SabqponmabcryxxlaccszExkacctuvwjabdefghi".to_vec();
        let grid = Grid::<8, 5>::new(data);
        let (shortest_path, _cost) = grid.shortest_path_from_any_pos().unwrap();

        assert_eq!(29, shortest_path.len() - 1)
    }
}
