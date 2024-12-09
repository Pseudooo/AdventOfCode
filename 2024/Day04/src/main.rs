use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./data.txt")
        .expect("Unable to read file");
    println!("Result: {}", count_matches(&file_contents));
}

fn count_matches(text: &str) -> i32 {
    let directions = [
        (0, -1), (0, 1), (-1, 0), (1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1),
    ];

    let mut total = 0;
    let lines: Vec<Vec<char>> = text.lines().map(|s| s.chars().collect()).collect();

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {

            if char != &'X' {
                continue;
            }

            let pos = (j as i32, i as i32);
            for direction in directions {
                let (x_end, y_end) = walk(pos, direction, 3);

                if x_end < 0 || y_end < 0 {
                    continue;
                } else if x_end >= lines.len() as i32 || y_end >= lines.len() as i32 {
                    continue;
                } else if lines[y_end as usize][x_end as usize] != 'S' {
                    continue;
                }

                let (x_m, y_m) = walk(pos, direction, 1);
                if lines[y_m as usize][x_m as usize] != 'M' {
                    continue;
                }

                let (x_a, y_a) = walk(pos, direction, 2);
                if lines[y_a as usize][x_a as usize] != 'A' {
                    continue;
                }

                total += 1;
            }
        }
    }

    return total;
}

fn walk(i0: (i32, i32), direction: (i32, i32), dist: i32) -> (i32, i32) {
    (i0.0 + direction.0 * dist, i0.1 + direction.1 * dist)
}

#[cfg(test)]
mod tests {
    use crate::count_matches;

    #[test]
    fn it_works() {
        let example = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = 18;
        assert_eq!(result, count_matches(example));
    }
}
