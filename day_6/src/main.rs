type Point = (usize, usize);
type Grid = Vec<Point>;

fn dist(a : &Point, b: &Point) -> usize {
    ((b.0 as i32 - a.0 as i32).abs() + (b.1 as i32 - a.1 as i32).abs()) as usize
}

fn total_dist(p: &Point, points: &Grid) -> usize {
    points.iter().map(|t| dist(p,t)).sum()
}

fn get_points(input_str: &str) -> Vec<Point> {
    input_str.lines().map(|l|{
        (l.split(',').nth(0).unwrap().trim().parse::<usize>().unwrap(), 
        l.split(',').nth(1).unwrap().trim().parse::<usize>().unwrap())
    }).collect()
}

fn has_finite_boundary(point: &Point, points: &Vec<Point>) -> bool {
    points.iter().any(|(x,_)| x < &point.0) && points.iter().any(|(x,_)| x > &point.0) &&
    points.iter().any(|(_,y)| y < &point.1) && points.iter().any(|(_,y)| y > &point.1)
}

fn create_grid(min: Point, max: Point) -> Grid {
    (min.0..max.0).map(|x| (min.1..max.1).map(|y| (x, y)).collect::<Grid>()).flatten().collect::<Grid>()
}

fn get_closest_point(p: &Point, points: &Grid) -> Option<Point> {
    let distances = points.iter().map(|t| (t, dist(p, t))).collect::<Vec<(&Point, usize)>>();
    let min_dist = distances.iter().min_by_key(|(_,d)| d).unwrap();
    if distances.iter().filter(|(_,d)| d == &min_dist.1).count() > 1 {
        None
    } else  {
        Some(*min_dist.0)
    }
}

fn part_1_solve(input_str: &str) -> usize {
    let points = get_points(input_str);
    let min_y = points.iter().min_by_key(|(_,y)| y).unwrap().1;
    let min_x = points.iter().min_by_key(|(x,_)| x).unwrap().0;
    let max_y = points.iter().max_by_key(|(_,y)| y).unwrap().1;
    let max_x = points.iter().max_by_key(|(x,_)| x).unwrap().0;

    let filtered = points.iter().filter(|&p| has_finite_boundary(&p, &points)).collect::<Vec<&Point>>();

    filtered.into_iter().map(|f| {
        create_grid((min_x, min_y), (max_x, max_y)).into_iter().filter(|p| {
            match get_closest_point(&p, &points) {
                Some(x) => x == *f,
                None => false
            }
        }).count()
    }).max().unwrap()
}

fn part_2_solve(input_str: &str, max_dist: usize) -> usize {
    let points = get_points(input_str);
    let min_y = points.iter().min_by_key(|(_,y)| y).unwrap().1;
    let min_x = points.iter().min_by_key(|(x,_)| x).unwrap().0;
    let max_y = points.iter().max_by_key(|(_,y)| y).unwrap().1;
    let max_x = points.iter().max_by_key(|(x,_)| x).unwrap().0;

    create_grid((min_x, min_y), (max_x, max_y)).into_iter().filter(|p| {
        total_dist(&p, &points) < max_dist
    }).count()
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
    println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt"), 10000));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1_solve(include_str!("../input/test_input_1.txt")), 17);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2_solve(include_str!("../input/test_input_1.txt"), 32), 16);
}