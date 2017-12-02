use std::num::ParseIntError;
use std::fs::File;
use std::io::Read;

pub type Rows = Vec<Vec<u32>>;

pub fn parse_rows(s: &str) -> Result<Rows, ParseIntError> {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>())
                .collect::<Result<Vec<u32>, ParseIntError>>()
        })
        .collect()
}

pub fn checksum(rows: &Rows) -> u32 {
    rows.iter().fold(0, |acc, row| {
        let max = row.iter().max().expect("Unable to get max");
        let min = row.iter().min().expect("Unable to get min");
        acc + max - min
    })
}

pub fn checksum_part2(rows: &Rows) -> u32 {
    rows.iter().fold(0, |acc, row| {
        let unique_remainder = row.iter()
            .filter_map(|&n| {
                row.iter()
                    .filter_map(|&elem| {
                        let remainder = elem / n;
                        if elem != n && (remainder * n) == elem {
                            Some(remainder)
                        } else {
                            None
                        }
                    })
                    .nth(0)
            })
            .nth(0)
            .expect("unique remainder");
        acc + unique_remainder
    })
}

fn main() {
    let mut input_file = File::open("src/bin/day2.input").expect("day2.input");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("read");
    let rows = parse_rows(&input).expect("rows");
    println!("Checksum Part1: {:?}", checksum(&rows));
    println!("Checksum Part2: {:?}", checksum_part2(&rows));
}


#[cfg(test)]
mod tests {
    use {checksum, parse_rows, checksum_part2};
    #[test]
    fn day2_tests() {
        let s = "5 1 9 5\n7 5 3\n2 4 6 8\n";
        let rows = parse_rows(s).expect("rows");
        println!("{:?}", rows);
        assert_eq!(checksum(&rows), 18);
    }

    #[test]
    fn day2_tests_part2() {
        let s = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        let rows = parse_rows(s).expect("rows");
        println!("{:?}", rows);
        assert_eq!(checksum_part2(&rows), 9);
    }
}
