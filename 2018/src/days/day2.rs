use advent_of_code_2018::AoCDay;
use advent_of_code_2018::Puzzle;
use std::collections::HashMap;

/*
--- Day 2: Inventory Management System ---

You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.

Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of suit that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"

"Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk too far away to hear any more.

Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates (your puzzle input).

To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID containing exactly two of any letter and then separately counting those with exactly three of any letter. You can multiply those two counts together to get a rudimentary checksum and compare it to what your device predicts.

For example, if you see the following box IDs:

    abcdef contains no letters that appear exactly two or three times.
    bababc contains two a and three b, so it counts for both.
    abbcde contains two b, but no letter appears exactly three times.
    abcccd contains three c, but no letter appears exactly two times.
    aabcdd contains two a and two d, but it only counts once.
    abcdee contains two e.
    ababab contains three a and three b, but it only counts once.

Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.

What is the checksum for your list of box IDs?

Your puzzle answer was 5904.

The first half of this puzzle is complete! It provides one gold star: *
--- Part Two ---

Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.

The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:

abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz

The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.

What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)

*/
pub fn day() -> AoCDay {
    let input = include_str!("inputs/day2.txt");

    AoCDay::new(2, "Inventory Management System")
        .add_puzzle(Puzzle::new(input, Box::new(puzzle1)).complete())
        .add_puzzle(Puzzle::new(input, Box::new(puzzle2)).complete())
}

fn parse_input(input: String) -> Vec<BoxID> {
    input.lines().map(|line| BoxID::new(line.to_string())).collect()
}

fn puzzle1(input: String) -> String {
    let input = parse_input(input);

    let mut two_cnt = 0;
    let mut three_cnt = 0;
    for b in input {
        if b.two_letters_match {
            two_cnt += 1;
        }
        if b.three_letters_match {
            three_cnt += 1;
        }
    }

    format!("{}", (two_cnt * three_cnt))
}

fn puzzle2(input: String) -> String {

    for line_a in input.lines() {
        for line_b in input.lines() {
            if line_a.eq(line_b) {
                continue
            }

            let mut common = String::new();

            for i in 0..line_a.len() {
                let char_a = &line_a[i..i+1];
                let char_b = &line_b[i..i+1];
                if char_a.eq(char_b) {
                    common.push(char_a.chars().next().unwrap());
                }
            }

            if common.len() == (line_a.len() - 1) {
                return common;
            }
        }
    }

    format!("none :(")
}

#[derive(Debug)]
struct BoxID {
    pub id: String,
    pub two_letters_match: bool,
    pub three_letters_match: bool,
}

impl BoxID {
    pub fn new(boxid: String) -> BoxID {
        BoxID {
            two_letters_match: has_letters_of_count(&boxid, 2),
            three_letters_match: has_letters_of_count(&boxid, 3),
            id: boxid,
        }
    }
}

fn has_letters_of_count(input: &String, count: i32) -> bool {
    let mut map = HashMap::new();
    for l in input.chars() {
        if ! map.contains_key(&l) {
            map.insert(l, 0);
        }

        map.insert(l, map.get(&l).unwrap() + 1);
    }

    for (_letter, cnt) in map.iter() {
        if *cnt == count {
            return true;
        }
    }

    return false;
}