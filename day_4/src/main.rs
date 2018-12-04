extern crate chrono;

use std::collections::HashMap;
use chrono::{NaiveDateTime};

#[derive(Debug)]
enum Event {
    None,
    Begin(i32),
    Sleep(NaiveDateTime),
    Wake(NaiveDateTime)
}

/*
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
*/

fn parse_time(time_str: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(time_str, "%F %R").unwrap()
}

fn parse_event(event_str: &str, at_time: NaiveDateTime) -> Event {
    match event_str.chars().nth(0).unwrap() {
        'G' => {
            Event::Begin(event_str.split_whitespace().nth(1).unwrap().trim_matches('#').parse::<i32>().unwrap())
        },
        'f' => Event::Sleep(at_time),
        'w' => Event::Wake(at_time),
        _ => Event::None
    }
}

fn get_sleep_map(start_time: &NaiveDateTime, end_time: &NaiveDateTime) -> Vec<i32> {
    
}

fn part_1_solve(input_str: &str) -> i32 {
    let mut entries = input_str.lines().map(|l| {
                        let splits: Vec<&str> = l.split(|c| c == '[' || c == ']').collect();
                        let timestamp = parse_time(splits[1]);
                        (timestamp, parse_event(splits[2].trim(), timestamp))
                    }).collect::<Vec<(NaiveDateTime, Event)>>();
                    
    entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut current_guard = 0;
    let mut prev_event = Event::None;
    let mut guard_map = HashMap::new();

    for entry in entries {
        match entry.1 {
            Event::Begin(guard_num) => {
                //Start a new shift
                current_guard = guard_num;
            },
            Event::Wake(wake_time) => {
                match prev_event {
                    Event::Sleep(sleep_time) => {
                        let time_asleep_mins = (wake_time - sleep_time).num_minutes();
                        let minute_map = *guard_map.entry(current_guard).or_insert(vec![0; len]);
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        prev_event = entry.1;
    }

    println!("{:?}", guard_map);

    0
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1_solve(include_str!("../input/test_input_1.txt")), 240);
}