fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    dbg!(result);
}

fn part2(input: &str) -> i32 {
    let digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let split = input.split("\n");
    let mut sum = 0;
    for line in split {
        if line.is_empty() { break; }
        // println!("===");
        let mut first_digit = -1;
        let mut last_digit = -1;
        let mut digit_construct = "".to_string();
        for char in line.chars() {
            // println!("{}", char.is_digit(10))
            if char.is_digit(10) {
                let current_digit = char.to_digit(10).unwrap() as i32;
                if first_digit == -1 {
                    first_digit = current_digit
                }

                last_digit = current_digit;

                digit_construct = "".to_string();
            } else {
                digit_construct.push(char);
                // println!("{} | {}", char, digit_construct);
                let mut has_matched_any = false;
                for digit in digits {
                    if digit == digit_construct {
                        let current_digit = digits.iter().position(|&x| x == digit_construct);
                        match current_digit {
                            Some(x) => {
                                if first_digit == -1 {
                                    first_digit = x as i32;
                                }
                
                                last_digit = x as i32;

                                digit_construct = char.to_string();

                                has_matched_any = true;

                                // println!("M {} {}", first_digit, last_digit);
                            },
                            None => panic!()
                        }
                    } else if digit.starts_with(&digit_construct) {
                        has_matched_any = true
                    }
                }
                if !has_matched_any {
                    digit_construct = digit_construct[1..].to_string()
                }
            }
        }
        sum += (first_digit * 10) + last_digit;
        // println!("{} {}", first_digit, last_digit);
    }

    return sum;
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {
        let result = super::part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, 281);
    }
}