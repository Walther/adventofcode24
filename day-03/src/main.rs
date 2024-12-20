use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let parsed = parse(INPUT);

    let value = part1(&parsed);
    println!("Part 1: {value}");

    let value = part2(&parsed);
    println!("Part 2: {value}");
}

type ParsedData = Vec<Instruction>;
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

fn parse(input: &str) -> ParsedData {
    let mut instructions: Vec<Instruction> = Vec::new();
    let instruction_regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)")
        .expect("Failed to construct a regular expression");
    let mul_regex =
        Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to construct a regular expression");

    for instruction in instruction_regex.find_iter(input).map(|m| m.as_str()) {
        match instruction {
            "do()" => {
                instructions.push(Instruction::Do);
                continue;
            }
            "don't()" => {
                instructions.push(Instruction::Dont);
                continue;
            }
            _ => {
                let (_full, [mul_left, mul_right]) = mul_regex
                    .captures(instruction)
                    .expect("Unable to parse mul instruction")
                    .extract();
                let l: usize = mul_left.parse().expect("Unable to parse number");
                let r: usize = mul_right.parse().expect("Unable to parse number");
                instructions.push(Instruction::Mul(l, r));
            }
        }
    }

    instructions
}

fn part1(data: &ParsedData) -> usize {
    let mut total = 0;
    for instruction in data {
        match instruction {
            Instruction::Mul(l, r) => total += l * r,
            _ => continue,
        }
    }
    total
}

fn part2(data: &ParsedData) -> usize {
    let mut total = 0;
    let mut enabled = true;
    for instruction in data {
        match instruction {
            Instruction::Mul(l, r) => {
                if enabled {
                    total += l * r;
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }
    total
}

#[cfg(test)]
mod integration {
    const INPUT_1: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT_2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1() {
        let parsed = crate::parse(INPUT_1);
        let value = crate::part1(&parsed);
        let expected = 161;
        assert_eq!(value, expected);
    }

    #[test]
    fn part2() {
        let parsed = crate::parse(INPUT_2);
        let value = crate::part2(&parsed);
        let expected = 48;
        assert_eq!(value, expected);
    }
}
