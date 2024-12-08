use common::read_lines;

fn main() {
    let lines_res = read_lines("./data.txt");
    if let Ok(lines) = lines_res {
        let mut safe_count = 0;
        for line in lines.flatten() {
            let processed_line = process_line(&line);
            if is_safe(&processed_line) {
                safe_count += 1;
            }
        }
        println!("Found {safe_count} safe entries");
    } else {
        println!("Failed to load data file");
    }
}

fn is_safe(values: &Vec<i32>) -> bool {
    let is_increasing = values[1] > values[0];
    let mut last_val: i32 = values[0];
    for curr_val in &values[1..] {

        if *curr_val == last_val {
            return false;
        } else if *curr_val > last_val && !is_increasing {
            return false;
        } else if *curr_val < last_val && is_increasing {
            return false;
        } else if (curr_val - last_val).abs() > 3 {
            return false;
        }

        last_val = *curr_val;
    }
    return true;
}

fn process_line(line: &str) -> Vec<i32> {
    let split: Vec<&str> = line.split(" ").collect();
    let mut values = Vec::with_capacity(split.len());
    for value in split {
        match value.parse::<i32>() {
            Ok(i32_value) => values.push(i32_value),
            Err(_) => {
                println!("Failed parse {value} as i32");
            }
        }
    }
    values
}

