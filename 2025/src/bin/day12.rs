use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader, Read},
};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input);
    p1(&mut input);
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Cell::Some,
            '.' => Cell::None,
            c => panic!("present contained {c}"),
        }
    }
}

impl From<&Cell> for char {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::Some => '#',
            Cell::None => '.',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    None,
    Some,
}

type Arrangement = Vec<Vec<Cell>>;
type Grid = Vec<Vec<Cell>>;

struct Present {
    id: usize,
    arrangements: HashSet<Arrangement>,
    arrangements_for_grid: HashMap<Grid, Vec<Arrangement>>,
}

impl Present {
    fn new(id: usize, init: Arrangement) -> Self {
        let mut arrangements = HashSet::new();
        for mut shape in [
            flip_ns(&init),
            flip_we(&init),
            flip_sw_ne(&init),
            flip_nw_se(&init),
            init,
        ] {
            for _ in 0..4 {
                shape = rotate90(&shape);
                arrangements.insert(shape.clone());
            }
        }

        Self {
            id,
            arrangements,
            arrangements_for_grid: HashMap::new(),
        }
    }

    fn fitting_arrangments(&mut self, grid: &Grid) -> Vec<Arrangement> {
        if let Some(arrangements) = self.arrangements_for_grid.get(grid) {
            return arrangements.clone();
        }

        let mut fitting_arrangements = Vec::new();
        for arrangement in self.arrangements.iter() {
            if arrangement_fits(grid, arrangement) {
                fitting_arrangements.push(arrangement.clone());
            }
        }

        self.arrangements_for_grid
            .insert(grid.clone(), fitting_arrangements.clone());
        return fitting_arrangements;
    }
}

fn print_arrangement(arr: &Arrangement) {
    for row in arr {
        for c in row {
            print!("{}", char::from(c))
        }
        print!("\n");
    }
}

fn print_grid(grid: &Grid) {
    for row in grid {
        for c in row {
            print!("{}", char::from(c))
        }
        print!("\n");
    }
}

fn flip_ns(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arri) in (0..=2).enumerate() {
        for (resj, arrj) in (0..=2).rev().enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }
    res
}

fn flip_we(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arri) in (0..=2).rev().enumerate() {
        for (resj, arrj) in (0..=2).enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }
    res
}

fn flip_sw_ne(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arrj) in (0..=2).rev().enumerate() {
        for (resj, arri) in (0..=2).rev().enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }
    res
}

fn flip_nw_se(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arrj) in (0..=2).enumerate() {
        for (resj, arri) in (0..=2).enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }
    res
}

fn rotate90(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arrj) in (0..=2).rev().enumerate() {
        for (resj, arri) in (0..=2).enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }

    res
}

fn p1(input: &[u8]) {
    let mut buf_reader = BufReader::new(input);
    let mut line = String::new();
    let mut presents: HashMap<usize, Present> = HashMap::new();
    while let Ok(len) = buf_reader.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let Some(colon_idx) = line.find(":") else {
            panic!("id should contain colon")
        };

        let Ok(id): Result<_, _> = line[..colon_idx].parse() else {
            break;
        };

        line.clear();
        let mut init: Arrangement = Vec::new();
        while let Ok(len) = buf_reader.read_line(&mut line) {
            if len == 1 {
                break;
            }
            init.push(line.trim().chars().map(|c| Cell::from(c)).collect());
            line.clear();
        }

        presents.insert(id, Present::new(id, init));

        line.clear();
    }

    let mut total = 0;
    loop {
        let Some(x_idx) = line.find("x") else { break };
        let nrows: usize = line[0..x_idx].parse().unwrap();
        let Some(colon_idx) = line.find(":") else {
            panic!("colon is expected: `{line}`");
        };
        let ncols: usize = line[(x_idx + 1)..colon_idx].parse().unwrap();

        let mut missing_presents: Vec<usize> = Vec::new();
        let remainder = &line[colon_idx + 2..];
        for (pid, npresents) in remainder.split_whitespace().enumerate() {
            let npresents = npresents.parse().unwrap();
            if npresents == 0 {
                continue;
            }

            for _ in 0..npresents {
                missing_presents.push(pid);
            }
        }

        total += if missing_presents.len() * 9 <= ncols * nrows {
            1
        } else {
            0
        };

        // println!(
        //     "{:?}",
        //     find_layout(grid, missing_presents, &mut presents, 0)
        // );
        line.clear();
        buf_reader.read_line(&mut line);
    }
    println!("p1: {total}");
}

type Position = (usize, usize);

fn arrangement_fits(grid: &Grid, arrangement: &Arrangement) -> bool {
    let mut fits = true;
    for i in 0..arrangement.len() {
        for j in 0..arrangement[0].len() {
            if arrangement[i][j] == Cell::Some && grid[i][j] == Cell::Some {
                fits = false
            }

            if !fits {
                break;
            }
        }
        if !fits {
            break;
        }
    }
    fits
}

fn fill_grid(grid: &mut Grid, pos: &Position, arrangement: &Arrangement) {
    for i in 0..3 {
        for j in 0..3 {
            if let Cell::None = arrangement[i][j] {
                continue;
            }
            grid[pos.0 + i][pos.1 + j] = arrangement[i][j];
        }
    }
}

fn sub_grid(grid: &Grid, (i, j): &(usize, usize)) -> Grid {
    let mut sub_grid = Vec::with_capacity(3);
    for i in *i..(*i + 3) {
        let mut row = Vec::with_capacity(3);
        for j in *j..(*j + 3) {
            row.push(grid[i][j]);
        }
        sub_grid.push(row);
    }
    return sub_grid;
}

fn find_layout(
    grid: Grid,
    missing_presents: HashMap<usize, u64>,
    presents: &mut HashMap<usize, Present>,
    depth: u64,
) -> bool {
    if missing_presents.len() == 0 {
        for row in grid {
            for cell in row {
                print!("{}", char::from(&cell));
            }
            print!("\n");
        }
        return true;
    }

    for (pid, _) in missing_presents.iter() {
        let present = presents.get_mut(pid).unwrap();
        let mut fits = false;
        for i in 0..=(grid.len() - 3) {
            for j in 0..=(grid[0].len() - 3) {
                if let Cell::Some = grid[i][j] {
                    continue;
                }

                let pos = (i, j);
                let sub_grid = sub_grid(&grid, &pos);
                fits = !present.fitting_arrangments(&sub_grid).is_empty();
                if fits {
                    break;
                }
            }
            if fits {
                break;
            }
        }
        if !fits {
            // println!("Early break");
            return false;
        }
    }

    for (pid, _) in missing_presents.iter() {
        let mut missing_presents = missing_presents.clone();
        let cnt = missing_presents.get_mut(pid).unwrap();
        *cnt -= 1;
        if *cnt == 0 {
            missing_presents.remove(pid);
        }

        for i in 0..=(grid.len() - 3) {
            for j in 0..=(grid[0].len() - 3) {
                if let Cell::Some = grid[i][j] {
                    continue;
                }

                let pos = (i, j);
                let sub_grid = sub_grid(&grid, &pos);
                let present = presents.get_mut(pid).unwrap();
                let fitting_arrangements = present.fitting_arrangments(&sub_grid);
                for arrangement in fitting_arrangements {
                    let mut grid = grid.clone();
                    // print_grid(&grid);
                    // println!("******");
                    fill_grid(&mut grid, &pos, &arrangement);
                    // print_grid(&grid);
                    // println!("******\n");
                    println!("{depth} {pos:?}");
                    if find_layout(grid, missing_presents.clone(), presents, depth + 1) {
                        return true;
                    }
                }
            }
        }
    }

    return false;
}
