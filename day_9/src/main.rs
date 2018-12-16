use std::collections::VecDeque;

fn rotate(circle: &mut VecDeque<u32>, rotator: isize) {
    if rotator > 0 {
        for _ in 0..rotator {
            let rotate_val = circle.pop_front().unwrap();
            circle.push_back(rotate_val);
        }
    } else {
        for _ in 0..-rotator {
            let rotate_val = circle.pop_back().unwrap();
            circle.push_front(rotate_val);
        }
    }
}

fn get_high_score(num_players: u32, final_marble_score: u32) -> u32 {
    let mut scores = vec!(0;num_players as usize);
    let mut marbles = VecDeque::new();
    marbles.push_back(0);

    for t in 1..=final_marble_score {
        let player = t % num_players;
        if t % 23 == 0 {
            rotate(&mut marbles, -7);
            scores[player as usize] += t + marbles.pop_back().unwrap();
            rotate(&mut marbles, 1);
        } else {
            rotate(&mut marbles, 1);
            marbles.push_back(t);
        }
    }

    *scores.iter().max().unwrap()
}

//464 players; last marble is worth 71730 points
fn main() {
    println!("Part 1: {}", get_high_score(464, 71730));
    println!("Part 2: {}", get_high_score(464, 71730*100));
}

#[test]
fn part_1_test() {
    assert_eq!(get_high_score(9, 25), 32);
    assert_eq!(get_high_score(10, 1618), 8317);
    assert_eq!(get_high_score(13, 7999), 146373);
    assert_eq!(get_high_score(17, 1104), 2764);
    assert_eq!(get_high_score(21, 6111), 54718);
    assert_eq!(get_high_score(30, 5807), 37305);
}