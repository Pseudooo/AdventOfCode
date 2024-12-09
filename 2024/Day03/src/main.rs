use regex::Regex;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./data.txt")
        .expect("Unable to read file");
    println!("Result: {}", process(&file_contents));
}

fn process(text: &str) -> i32 {
    let mut total = 0;

    let re = Regex::new(r"mul\((\d){1,3},(\d){1,3}\)").unwrap();
    for (capture, _) in re.captures_iter(text).map(|x| x.extract::<2>()) {
        let digits = &capture[4..capture.len() - 1];
        total += match digits.split_once(',') {
            Some((left, right)) => left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap(),
            None => panic!("Something is wrong...")
        };
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = 161;
        assert_eq!(process(example), result);
    }
}