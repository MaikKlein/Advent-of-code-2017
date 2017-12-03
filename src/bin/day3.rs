#[derive(Debug, Copy, Clone)]
pub struct Grid {
    pub value: u32,
    /// The euclidean distance to the center
    pub euclid_dist: u32,
    /// Largest square root of the bottom right number of a row. For example 3, 5, 7 etc.
    pub largest_sqroot: u32,
}

impl Grid {
    pub fn from_value(n: u32) -> Grid {
        let (largest_sqroot, euclid_dist) = (0..)
            .map(|i| i * 2 + 1)
            .take_while(|i| i * i < n)
            .zip(1..)
            .last()
            .map(|(sq, idx)| (sq + 2, idx))
            .unwrap_or((1, 0));
        Grid {
            value: n,
            euclid_dist,
            largest_sqroot,
        }
    }

    /// The number of elements in a spiral row. For example the first row starts from 2 to 9 which contains 8 elements.
    pub fn row_elements(&self) -> u32 {
        self.previous_sqroot()
            .map(|prev_sq| self.largest_sqroot.pow(2) - prev_sq)
            .unwrap_or(1)
    }

    /// The previous sqroot. For example if the current value is 21, then the current sqroot is
    /// 5, and the previous sqroot is 2.
    pub fn previous_sqroot(&self) -> Option<u32> {
        u32::checked_sub(self.largest_sqroot, 2).map(|n| n.pow(2))
    }

    pub fn manhatten_dist(&self) -> u32 {
        if let Some(prev_sq) = self.previous_sqroot() {
            // A square as for sides. We use this to the center element of each side.
            let elemnts_per_side = self.row_elements() / 4;
            // The amount of steps to the center element of a spiral row.
            let steps_row = (0..4)
                .map(|i| {
                    // We calculate the difference of the center element of every side with out value.
                    // The center element is the `elements_per_side` but with an offset of the `euclid_dist`.
                    let offset = (prev_sq + self.euclid_dist + i * elemnts_per_side) as i32;
                    i32::abs(offset - self.value as i32) as u32
                })
                // We choose the smallest amount of steps.
                .min()
                .expect("min");
            steps_row + self.euclid_dist
        } else {
            0
        }
    }
}

fn main() {
    let d = Grid::from_value(265149).manhatten_dist();
    println!("Part 1{}", d);
}


#[cfg(test)]
mod tests {
    use Grid;
    #[test]
    fn test_day3() {
        assert_eq!(Grid::from_value(1).manhatten_dist(), 0);
        assert_eq!(Grid::from_value(6).manhatten_dist(), 1);
        assert_eq!(Grid::from_value(9).manhatten_dist(), 2);
        assert_eq!(Grid::from_value(12).manhatten_dist(), 3);
        assert_eq!(Grid::from_value(21).manhatten_dist(), 4);
        assert_eq!(Grid::from_value(25).manhatten_dist(), 4);
        assert_eq!(Grid::from_value(1024).manhatten_dist(), 31);
    }
}
