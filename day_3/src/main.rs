type Point = (u32, u32);
type Grid = Vec<(u32, u32)>;

#[derive(Debug)]
struct Rect {
    p1 : Point,
    p2 : Point
}

#[derive(Debug)]
struct Claim {
    claim_num : u32,
    area : Rect
}

impl Claim {
    fn new(claim_desc : &str) -> Claim {
        let split: Vec<&str> = claim_desc.splitn(4, |c| c == '#' || c == '@' || c == ':').map(|s| s.trim()).collect();
        let coords: Vec<&str> = split[2].splitn(2, ',').collect();
        let size: Vec<&str> = split[3].splitn(2, 'x').collect();

        Claim {
            claim_num : split[1].parse::<u32>().unwrap(),
            area : Rect {
                p1 : (coords[0].parse::<u32>().unwrap(), coords[1].parse::<u32>().unwrap()),
                p2 : (coords[0].parse::<u32>().unwrap() + size[0].parse::<u32>().unwrap(), coords[1].parse::<u32>().unwrap() + size[1].parse::<u32>().unwrap())
            }
        }
    }

    fn contains_point(&self, p : Point) -> bool {
        (p.0 > self.area.p1.0 && p.0 <= self.area.p2.0) &&
        (p.1 > self.area.p1.1 && p.1 <= self.area.p2.1)
    }

    fn intersects(&self, c: &Claim) -> bool {
        !(c.area.p1.0 >= self.area.p2.0 || c.area.p2.0 <= self.area.p1.0 || c.area.p1.1 >= self.area.p2.1 || c.area.p2.1 <= self.area.p1.1)
    }
}

fn create_grid(size: Point) -> Grid {
    (0..size.0).map(|x| (0..size.1).map(|y| (x, y)).collect::<Grid>()).flatten().collect::<Grid>()
}

fn has_overlap(p: Point, claims: &Vec<Claim>) -> bool {
    claims.into_iter().filter(|c| c.contains_point(p)).count() > 1
}

fn get_num_overlaps(grid_size: Point, claims: &Vec<Claim>) -> usize {
    create_grid(grid_size).into_iter().fold(0, |overlaps, p| {
        overlaps + (has_overlap(p, claims) as usize)
    })
}

fn part_1_solve(input_str: &str, grid_size: Point) -> usize {
    let claims : Vec<Claim> = input_str.lines().map(|claim_line| Claim::new(claim_line)).collect();
    get_num_overlaps(grid_size, &claims)
}

fn compare(c: &Claim, v: &Vec<Claim>) -> bool {
    v.into_iter()
    .filter(|x| {
        x.claim_num != c.claim_num
    }).all(|&ref x| {
        !x.intersects(c)
    })
}

fn part_2_solve(input_str: &str) -> u32 {
    let claims : Vec<Claim> = input_str.lines().map(|claim_line| Claim::new(claim_line)).collect();
    for i in 0..claims.len() {
        if compare(&claims[i], &claims) {
            return claims[i].claim_num;
        }
    }

    unreachable!()
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt"), (1000,1000)));
    println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt")));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1_solve(include_str!("../input/test_input_1.txt"), (8,8)), 4);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2_solve(include_str!("../input/test_input_1.txt")), 3);
}