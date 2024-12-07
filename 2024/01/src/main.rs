use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {

    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./data.txt") {
        for line in lines.flatten() {
            let split: Vec<&str> = line.split("   ").collect();
            parse_and_push(&mut a, split[0]);
            parse_and_push(&mut b, split[1]);
        }

        a.sort();
        b.sort();

        let mut sum = 0;
        for (left, right) in a.iter().zip(b.iter()) {
            sum += (left - right).abs();
        }
        println!("Result: {sum}");
    } else {
        println!("Failed to open file!");
    }
}

fn parse_and_push(vec: &mut Vec<i32>, x: &str) {
    match x.parse::<i32>() {
        Ok(val) => vec.push(val),
        Err(_) => {
            println!("Something is wrong here...");
            dbg!(x);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
