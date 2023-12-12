#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    fn mins(&self, other: &Point) -> (isize, isize) {
        (self.x.min(other.x), self.y.min(other.y))
    }

    fn maxes(&self, other: &Point) -> (isize, isize) {
        (self.x.max(other.x), self.y.max(other.y))
    }
}

fn sum_distances(
    galaxies: &[Point],
    empty_rows: Vec<isize>,
    empty_cols: Vec<isize>,
    expansion: usize,
) -> usize {
    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other in &galaxies[i + 1..] {
            let dist = galaxy.manhattan_distance(other);

            let (min_x, min_y) = galaxy.mins(other);
            let (max_x, max_y) = galaxy.maxes(other);

            // count all rows that should be expanded where the path traverses
            // and multiply the count by the expansion value.
            let traversed_rows = empty_rows
                .iter()
                .filter(|row| min_y < **row && max_y > **row)
                .count()
                * expansion;

            // same for columns
            let traversed_cols = empty_cols
                .iter()
                .filter(|col| min_x < **col && max_x > **col)
                .count()
                * expansion;

            sum += dist + traversed_cols + traversed_rows;
        }
    }

    sum
}

fn main() {
    let input = include_str!("./input.txt");
    let grid = input
        .lines()
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, val)| **val == '#')
                .map(move |(x, _)| Point {
                    x: x as isize,
                    y: y as isize,
                })
        })
        .collect::<Vec<Point>>();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, f)| f.iter().all(|c| *c == '.'))
        .map(|(y, _)| y as isize)
        .collect::<Vec<isize>>();

    let empty_cols = (0..grid[0].len())
        .filter(|f| grid.iter().all(|r| r[*f] == '.'))
        .map(|x| x as isize)
        .collect::<Vec<isize>>();

    println!(
        "p1: {}",
        sum_distances(&galaxies, empty_rows, empty_cols, 999_999)
    )
}
