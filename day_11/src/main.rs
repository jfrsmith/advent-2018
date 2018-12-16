type Grid = Vec<(usize, usize)>;
type PowerGrid = Vec<Vec<i32>>;

fn precompute_sums(power_vals: &PowerGrid) -> PowerGrid {
    let mut precompute = vec!(vec!(0;power_vals[0].len());power_vals.len());
    for r in 0..power_vals[0].len() {
        precompute[0][r] = power_vals[0][r];
    }

    for c in 1..power_vals.len() {
        for r in 0..power_vals[0].len() {
            precompute[c][r] = power_vals[c][r] + precompute[c-1][r];
        }
    }

    for c in 0..power_vals.len() {
        for r in 1..power_vals[0].len() {
            precompute[c][r] += precompute[c][r-1];
        }
    }

    precompute
}

fn get_cell_power(cell: &(usize, usize), serial_num: i32) -> i32 {
    let rack_id = cell.0 as i32 + 10;
    let power_lvl = ((rack_id * cell.1 as i32) + serial_num) * rack_id;
    
    if power_lvl < 100 {
        -5
    } else {
        ((power_lvl / 100) % 10) - 5
    }
}

fn build_power_grid(size: usize, serial_num: i32) -> PowerGrid {
    (1..=size).map(|y| (1..=size).map(|x| {
        get_cell_power(&(x, y), serial_num)
    }).collect::<Vec<i32>>()).collect::<PowerGrid>()
}

fn part_1_solve(serial_num: i32) -> (usize, usize) {
    let power_grid = build_power_grid(300, serial_num);
    let precomputed = precompute_sums(&power_grid);

    let coords = (0..300).map(|x| (0..300).map(|y| (x, y)).collect::<Grid>())
    .flatten()
    .filter(|p| {
        p.0 + 2 < 300 && p.1 + 2 < 300
    })
    .max_by_key(|&p| {
        get_sum(&precomputed, p, (p.0+2, p.1+2))
    }).unwrap();

    (coords.1 + 1, coords.0 + 1)
}

fn part_2_solve(serial_num: i32, max_size: usize) -> ((usize, usize), usize) {
    let power_grid = build_power_grid(300, serial_num);
    let precomputed = precompute_sums(&power_grid);

    let coords = (0..300).map(|x| (0..300).map(|y| (x, y)).collect::<Grid>())
    .flatten()
    .map(|p| (1..=max_size).map(|s| (p, s)).collect::<Vec<((usize, usize), usize)>>())
    .flatten()
    .filter(|((x, y), s)| {
        x + s < 300 && y + s < 300
    })
    .max_by_key(|(p, s)| {
        get_sum(&precomputed, *p, (p.0+s, p.1+s))
    }).unwrap();

    (((coords.0).1 + 1, (coords.0).0 + 1), coords.1 + 1)
}

fn get_sum(precomputed: &PowerGrid, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut res = precomputed[end.0][end.1];
    if start.0 > 0 {
        res -= precomputed[start.0-1][end.1];
    }

    if start.1 > 0 {
        res -= precomputed[end.0][start.1-1];
    }

    if start.0 > 0 && start.1 > 0 {
        res += precomputed[start.0-1][start.1-1];
    }
    
    res
}

fn main() {
    println!("{:?}", part_1_solve(3463));
    println!("{:?}", part_2_solve(3463, 300));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1_solve(18), (33,45));
    assert_eq!(part_1_solve(42), (21,61));
}

#[test]
fn part_2_test() {
    assert_eq!(part_2_solve(18, 300), ((90,269), 16));
    assert_eq!(part_2_solve(42, 300), ((232,251), 12));
}

#[test]
fn cell_power_test() {
    assert_eq!(get_cell_power(&(3, 5), 8), 4);
    assert_eq!(get_cell_power(&(122,79), 57), -5);
    assert_eq!(get_cell_power(&(217,196), 39), 0);
    assert_eq!(get_cell_power(&(101,153), 71), 4);
}