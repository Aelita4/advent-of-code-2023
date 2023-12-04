fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    dbg!(result);
}

fn part1(input: &str) -> i32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let split = input.split("\n");
    let mut sum = 0;
    for line in split {
        if line.is_empty() { break; }
        let split_colon = line.split(": ").collect::<Vec<&str>>();
        let game_id = split_colon[0].split(" ").collect::<Vec<&str>>()[1].to_string().parse::<i32>().unwrap();
        let subsets = split_colon[1].split("; ").collect::<Vec<&str>>();
        let mut game_possible = true;
        for subset in subsets {
            let cubes = subset.split(", ").collect::<Vec<&str>>();
            for cube in cubes {
                let split_cube = cube.split(" ").collect::<Vec<&str>>();
                let amount = split_cube[0].to_string().parse::<i32>().unwrap();
                match split_cube[1] {
                    "red" => if amount > max_red { game_possible = false; },
                    "green" => if amount > max_green { game_possible = false; },
                    "blue" => if amount > max_blue { game_possible = false; },
                    _ => panic!(),
                }
            }
        }
        if game_possible { sum += game_id; }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {
        let result = super::part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 8);
    }
}