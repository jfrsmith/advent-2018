use std::io;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::fmt;

type Point = (usize, usize);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Facing {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Track {
    None,
    Horizontal,
    Vertical,
    Turn(char),
    Intersect
}

#[derive(Clone, Copy, Eq)]
struct Cart {
    curr_loc : Point,
    curr_facing : Facing,
    intersect_count : u32
}

impl fmt::Debug for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.curr_loc)
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.curr_loc == other.curr_loc
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        self.curr_loc.cmp(&other.curr_loc)
    }
}

impl Cart {
    fn new(cart_char: char, loc: Point) -> Cart {
        Cart {
            curr_loc : loc,
            curr_facing : match cart_char {
                '^' => Facing::Up,
                'v' => Facing::Down,
                '>' => Facing::Right,
                '<' => Facing::Left,
                _ => unreachable!()
            },
            intersect_count : 0
        }
    }

    fn tick(&self, railway: &Rail) -> Cart {
        let current_track = railway[self.curr_loc.0][self.curr_loc.1];

        let new_facing = match current_track {
            Track::Horizontal | Track::Vertical => self.curr_facing,
            Track::Turn(c) => self.turn(c),
            Track::Intersect => self.intersect_turn(),
            Track::None => unreachable!()
        };

        let new_loc = match new_facing {
            Facing::Up => (self.curr_loc.0-1, self.curr_loc.1),
            Facing::Down => (self.curr_loc.0+1, self.curr_loc.1),
            Facing::Left => (self.curr_loc.0, self.curr_loc.1-1),
            Facing::Right => (self.curr_loc.0, self.curr_loc.1+1)
        };

        Cart {
            curr_loc : new_loc,
            curr_facing : new_facing,
            intersect_count : if current_track == Track::Intersect {
                self.intersect_count + 1
            } else {
                self.intersect_count
            }
        }
    }

    fn turn(&self, turn_char: char) -> Facing {
        match turn_char {
            '\\' => match self.curr_facing {
                Facing::Up => Facing::Left,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Up,
                Facing::Right => Facing::Down
            },
            '/' => match self.curr_facing {
                Facing::Up => Facing::Right,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Down,
                Facing::Right => Facing::Up
            },
            _ => unreachable!()
        }
    }

    fn intersect_turn(&self) -> Facing {
        let intersect_mod = self.intersect_count % 3;

        if intersect_mod == 0 {
            //turn left
            match self.curr_facing {
                Facing::Up => Facing::Left,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
                Facing::Right => Facing::Up
            }
        } else if intersect_mod == 2 {
            //turn right
            match self.curr_facing {
                Facing::Up => Facing::Right,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Right => Facing::Down
            }
        } else {
            self.curr_facing
        }
    }
}

type Rail = Vec<Vec<Track>>;
type Carts = BTreeSet<Cart>;

fn print(carts: &Carts, railway: &Rail) {
    for y in 0..railway.len() {
        for x in 0..railway[y].len() {
            let found_cart = carts.iter().find(|&&c| c.curr_loc == (y, x));
            if let Some(c) = found_cart {
                match c.curr_facing {
                    Facing::Up => print!("^"),
                    Facing::Down => print!("v"),
                    Facing::Left => print!("<"),
                    Facing::Right => print!(">")
                }
            } else {
                match railway[y][x] {
                    Track::None => print!(" "),
                    Track::Horizontal => print!("-"),
                    Track::Vertical => print!("|"),
                    Track::Intersect => print!("+"),
                    Track::Turn(c) => print!("{}", c)
                }
            }
        }
        println!("");
    }
}

fn find_crash_location(input_str: &str) -> Point {
    let mut carts = Carts::new();

    let railway = input_str.lines().enumerate().map(|(y, l)| {
        l.chars().enumerate().map(|(x, c)| {
            match c {
                '|' => Track::Vertical,
                '-' => Track::Horizontal,
                '\\' | '/' => Track::Turn(c),
                '+' => Track::Intersect,
                '>' | '<' => {
                    carts.insert(Cart::new(c, (y, x)));
                    Track::Horizontal
                },
                '^' | 'v' => {
                    carts.insert(Cart::new(c, (y, x)));
                    Track::Vertical
                },
                _ => Track::None
            }
        }).collect::<Vec<Track>>()
    }).collect::<Rail>();

    let mut key = String::new();

    loop {
        //print(&carts, &railway);
        //println!("{:?}", carts);
        //io::stdin().read_line(&mut key);

        let mut new_carts = Carts::new();
        for c in carts {
            let next_tick = c.tick(&railway);
            if !new_carts.insert(next_tick) {
                return (next_tick.curr_loc.1, next_tick.curr_loc.0);
            }
        }

        carts = new_carts;
    }
}

fn main() {
    println!("{:?}", find_crash_location(include_str!("../input/input.txt")));
}

#[test]
fn part_1_test() {
    assert_eq!(find_crash_location(include_str!("../input/test_input_1.txt")), (7,3));
    assert_eq!(find_crash_location(include_str!("../input/test_input_2.txt")), (2,2));
}