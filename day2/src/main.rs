enum Choice {
    Rock,
    Paper,
    Scissors,
    None,
}

fn main() {
    let mut sum: i32 = 0;
    for line in include_str!("../input.txt").lines() {
        let line = line.split(" ").collect::<Vec<&str>>();
        let oponent = line[0].chars().nth(0).unwrap();
        let you = line[1].chars().nth(0).unwrap();
        let op_choice = createChoice(oponent);
        // let you_choice = createChoice(you);
        let you_choice = match(&op_choice, you) {
            (Choice::Rock, 'X') => Choice::Scissors,
            (Choice::Rock, 'Y') => Choice::Rock,
            (Choice::Rock, 'Z') => Choice::Paper,
            (Choice::Paper, 'X') => Choice::Rock,
            (Choice::Paper, 'Y') => Choice::Paper,
            (Choice::Paper, 'Z') => Choice::Scissors,
            (Choice::Scissors, 'X') => Choice::Paper,
            (Choice::Scissors, 'Y') => Choice::Scissors,
            (Choice::Scissors, 'Z') => Choice::Rock,
            _ => panic!("Invalid input"),
        };
        let result = match(op_choice, you_choice) {
            (Choice::Rock, Choice::Paper) => 6 + 2,
            (Choice::Rock, Choice::Scissors) => 0 + 3,
            (Choice::Paper, Choice::Rock) => 0 + 1,
            (Choice::Paper, Choice::Scissors) => 6 + 3,
            (Choice::Scissors, Choice::Rock) => 6 + 1,
            (Choice::Scissors, Choice::Paper) => 0 + 2,
            (Choice::Scissors, Choice::Scissors) => 3 + 3,
            (Choice::Paper, Choice::Paper) => 3 + 2,
            (Choice::Rock, Choice::Rock) => 3 + 1,
            _ => 0,
        };
        sum += result;
    }
    println!("Sum: {}", sum);
}

fn createChoice(c: char) -> Choice {
    match c {
        'A' | 'X' => Choice::Rock,
        'B' | 'Y' => Choice::Paper,
        'C' | 'Z' => Choice::Scissors,
        _ => panic!("Invalid choice"),
    }
}
