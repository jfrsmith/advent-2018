extern crate regex;

use regex::Regex;

#[derive(Debug)]
struct LightPoint {
    point : (i64, i64),
    velocity : (i64, i64)
}

impl LightPoint {
    fn new(line: &str) -> LightPoint {
        let re = Regex::new(r"position=<([\s\S\d]*?),([\s\S\d]*?)> velocity=<([\s\S\d]*?),([\s\S\d]*?)>").unwrap();
        let caps = re.captures(line).unwrap();

        LightPoint {
            point: (caps[1].trim().parse::<i64>().unwrap(),
                    caps[2].trim().parse::<i64>().unwrap()),
            velocity : (caps[3].trim().parse::<i64>().unwrap(),
                        caps[4].trim().parse::<i64>().unwrap())
        }
    }

    fn tick(&self) -> LightPoint {
        LightPoint {
            point: (self.point.0 + self.velocity.0, self.point.1 + self.velocity.1),
            velocity : self.velocity
        }
    }
}

type Sky = Vec<LightPoint>;

fn get_dims(sky: &Sky) -> ((i64, i64), (i64, i64)) {
    let min_x = sky.into_iter().map(|lp| lp.point.0).min().unwrap();
    let min_y = sky.into_iter().map(|lp| lp.point.1).min().unwrap();
    let max_x = sky.into_iter().map(|lp| lp.point.0).max().unwrap();
    let max_y = sky.into_iter().map(|lp| lp.point.1).max().unwrap();

    ((min_x, min_y), (max_x, max_y))
}

fn get_area(sky: &Sky) -> u64 {
    let ((min_x, min_y), (max_x, max_y)) = get_dims(sky);
    ((max_x - min_x).abs() * (max_y - min_y).abs()) as u64
}

fn render(sky: &Sky) {
    let ((min_x, min_y), (max_x, max_y)) = get_dims(sky);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if sky.into_iter().any(|lp| lp.point == (x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn run(input_str: &str) -> u64 {
    let mut sky = input_str.lines().map(|l| LightPoint::new(l)).collect::<Sky>();
    let mut area = get_area(&sky);
    let mut ticks = 0;

    loop {
        let new_sky = sky.iter().map(|lp| lp.tick()).collect::<Sky>();
        let new_area = get_area(&new_sky);
        if new_area > area {
            break;
        } else {
            sky = new_sky;
            area = new_area;
        }
        ticks += 1;
    }

    render(&sky);

    ticks
}

fn main() {
    println!("{}", run(include_str!("../input/input.txt")));
}

#[test]
fn part_2_test() {
    assert_eq!(run(include_str!("../input/test_input_1.txt")), 3);
}