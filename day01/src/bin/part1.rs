fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    dbg!(result);
}

fn part1(input: &str) -> i32 {
    let split = input.split("\n");
    let mut sum = 0;
    for line in split {
        if line.is_empty() { break; }
        let mut first_digit = -1;
        let mut last_digit = -1;
        for char in line.chars() {
            // println!("{}", char.is_digit(10))
            if char.is_digit(10) {
                let current_digit = char.to_digit(10).unwrap() as i32;
                if first_digit == -1 {
                    first_digit = current_digit
                }

                last_digit = current_digit;
            }
        }
        sum += (first_digit * 10) + last_digit;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {
        let result = super::part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }
}