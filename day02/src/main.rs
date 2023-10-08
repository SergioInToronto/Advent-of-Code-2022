use std::collections::HashMap;
use std::fs;


// const throw_score: HashMap<String, u32> = HashMap::from([
//     ("A", 1), // Rock
//     ("B", 2), // Paper
//     ("C", 3), // Scissors
//     ("X", 1), // Rock
//     ("Y", 2), // Paper
//     ("Z", 3), // Scissors
// ]);


fn outcome_score(p1: &String, p2: &String) -> u32 {
    // WIN: 6
    // DRAW: 3
    // LOSS: 0
    if (p1 == "B" && p2 == "X") || (p1 == "C" && p2 == "Y") || (p1 == "A" && p2 == "Z") {
        return 0;
    }
    if (p1 == "A" && p2 == "X") || (p1 == "B" && p2 == "Y") || (p1 == "C" && p2 == "Z") {
        return 3;
    }
    return 6;
}


fn part1(contents: String) -> u32 {
    let throw_score: HashMap<String, u32> = HashMap::from([
        ("A".to_string(), 1), // Rock
        ("B".to_string(), 2), // Paper
        ("C".to_string(), 3), // Scissors
        ("X".to_string(), 1), // Rock
        ("Y".to_string(), 2), // Paper
        ("Z".to_string(), 3), // Scissors
    ]);

    let mut total_score: u32 = 0;
    for row in contents.trim_end().split("\n") {
        // dbg!(row);
        // let (play1, play2) = row.split(" ").collect();
        // let parts: _ = row.split(" ").collect::<String>();
        // dbg!(parts);
        // let parts = row.split(" ").collect::<String>();
        // dbg!(parts);
        // println!("({}, {})", &parts[0], &parts[1]);

        let play1 = &row[0..1].to_string();
        let play2 = &row[2..3].to_string();
        let play_score = outcome_score(play1, play2) + throw_score.get(play2).unwrap();
        // println!("Play score: {} (Outcom: {}, my throw: {})", play_score, outcome_score(play1, play2), throw_score.get(play2).unwrap());
        total_score = total_score + play_score;
    }
    return total_score;
}


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    let total_score = part1(contents);
    println!("Part1: Total score: {}", total_score);
    // First run gave wrong answer: 9977. I had the round score backwards. Player2 is us, so "B X" is a loss.
}
