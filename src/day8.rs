use color_eyre::Result;
use itertools::Itertools;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day8.txt")?;
    let forest = Forest::<99, 99>::new(&data);

    let count = forest.num_trees_visible();
    println!("Part 1: {count} trees are visible");

    let best_score = forest.best_scenic_score();
    println!("Part 2: {best_score} is the best scenic score");

    Ok(())
}

struct Forest<const W: usize, const H: usize> {
    grid: Vec<u8>,
}

impl<const W: usize, const H: usize> Forest<W, H> {
    pub fn new(data: &str) -> Self {
        let grid = data
            .lines()
            .flat_map(|line| line.as_bytes().iter().copied())
            .map(|c| c - b'0')
            .collect::<Vec<u8>>();
        assert_eq!(grid.len(), W * H);
        Self { grid }
    }

    pub fn num_trees_visible(&self) -> usize {
        let mut count = 0;
        for i in 0..W {
            for j in 0..H {
                if self.is_visible(i, j) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn best_scenic_score(&self) -> usize {
        (0..W - 1)
            .cartesian_product(0..H - 1)
            .map(|(i, j)| self.scenic_score(i, j))
            .max()
            .unwrap()
    }

    pub fn scenic_score(&self, i: usize, j: usize) -> usize {
        let h = self.get(i, j);
        let top = self
            .look_top(i, j)
            .position(|t| t >= h)
            .map(|c| c + 1)
            .unwrap_or(j);
        let bottom = self
            .look_bottom(i, j)
            .position(|t| t >= h)
            .map(|c| c + 1)
            .unwrap_or(H - j - 1);
        let left = self
            .look_left(i, j)
            .position(|t| t >= h)
            .map(|c| c + 1)
            .unwrap_or(i);
        let right = self
            .look_right(i, j)
            .position(|t| t >= h)
            .map(|c| c + 1)
            .unwrap_or(W - i - 1);

        top * bottom * left * right
    }

    fn get(&self, i: usize, j: usize) -> u8 {
        self.grid[j * W + i]
    }

    fn look_top(&self, i: usize, j: usize) -> impl Iterator<Item = u8> + '_ {
        self.grid.iter().skip(i).step_by(W).take(j).rev().copied()
    }

    fn look_bottom(&self, i: usize, j: usize) -> impl Iterator<Item = u8> + '_ {
        self.grid.iter().skip(i).step_by(W).skip(j + 1).copied()
    }

    fn look_left(&self, i: usize, j: usize) -> impl Iterator<Item = u8> + '_ {
        self.grid.iter().skip(j * W).take(i).rev().copied()
    }

    fn look_right(&self, i: usize, j: usize) -> impl Iterator<Item = u8> + '_ {
        self.grid
            .iter()
            .skip(j * W)
            .skip(i + 1)
            .take(W - (i + 1))
            .copied()
    }

    fn is_visible_top(&self, i: usize, j: usize) -> bool {
        let h = self.get(i, j);
        // self.grid.iter().skip(i).step_by(W).take(j).all(|t| *t < h)
        self.look_top(i, j).all(|t| t < h)
    }

    fn is_visible_bottom(&self, i: usize, j: usize) -> bool {
        let h = self.get(i, j);
        self.look_bottom(i, j).all(|t| t < h)
    }

    fn is_visible_left(&self, i: usize, j: usize) -> bool {
        let h = self.get(i, j);
        self.look_left(i, j).all(|t| t < h)
    }

    fn is_visible_right(&self, i: usize, j: usize) -> bool {
        let h = self.get(i, j);
        self.look_right(i, j).all(|t| t < h)
    }

    fn is_visible(&self, i: usize, j: usize) -> bool {
        if self.is_edge(i, j) {
            true
        } else {
            self.is_visible_left(i, j)
                || self.is_visible_right(i, j)
                || self.is_visible_top(i, j)
                || self.is_visible_bottom(i, j)
        }
    }

    fn is_edge(&self, i: usize, j: usize) -> bool {
        i == 0 || j == 0 || i == W - 1 || j == H - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "30373
25512
65332
33549
35390";
        let forest = Forest::<5, 5>::new(data);
        // edge (row 0)
        assert!(forest.is_visible(0, 0));
        // row 1
        assert!(forest.is_visible(1, 1));
        assert!(forest.is_visible(2, 1));
        assert!(!forest.is_visible(3, 1));
        // row 2
        assert!(forest.is_visible(1, 2));
        assert!(!forest.is_visible(2, 2));
        assert!(forest.is_visible_right(3, 2));
        assert!(forest.is_visible(3, 2));
        // row3
        assert!(!forest.is_visible(1, 3));
        assert!(forest.is_visible(2, 3));
        assert!(!forest.is_visible(3, 3));

        assert_eq!(21, forest.num_trees_visible());
        assert_eq!(4, forest.scenic_score(2, 1));
        assert_eq!(8, forest.best_scenic_score());
    }
}
