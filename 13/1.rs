use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn draw_grid(cars: &Vec<Car>, grid: &Vec<Vec<char>>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let mut car = None;
            for c in cars {
                if (x, y) == c.pos {
                    car = Some(c.dir);
                    break;
                }
            }
            if !car.is_none() {
                print!("{}", car.unwrap());
            } else {
                print!("{}", col);
            }
        }
        println!("");
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right
}

#[derive(Debug)]
struct Car {
    dir: char,
    pos: (usize, usize),
    next_turn: Turn
}

fn tick(cars: &mut Vec<Car>, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    cars.sort_by(|a, b| a.pos.cmp(&b.pos));
    for i in 0..cars.len() {
        {
            let c = &mut cars[i];
            // move
            match c.dir {
                '>' => c.pos = (c.pos.0 + 1, c.pos.1),
                '<' => c.pos = (c.pos.0 - 1, c.pos.1),
                '^' => c.pos = (c.pos.0, c.pos.1 - 1),
                'v' => c.pos = (c.pos.0, c.pos.1 + 1),
                _ => panic!()
            }
            // maybe turn
            if grid[c.pos.1][c.pos.0] == '/' {
                c.dir = match c.dir {
                    '>' => '^',
                    '<' => 'v',
                    '^' => '>',
                    'v' => '<',
                    _ => panic!()
                }
            } else if grid[c.pos.1][c.pos.0] == '\\' {
                c.dir = match c.dir {
                    '>' => 'v',
                    '<' => '^',
                    '^' => '<',
                    'v' => '>',
                    _ => panic!()
                }
            } else if grid[c.pos.1][c.pos.0] == '+' {
                c.dir = match c.dir {
                    '>' => match c.next_turn {
                        Turn::Left => '^',
                        Turn::Right => 'v',
                        Turn::Straight => c.dir
                    },
                    '<' => match c.next_turn {
                        Turn::Left => 'v',
                        Turn::Right => '^',
                        Turn::Straight => c.dir
                    },
                    '^' => match c.next_turn {
                        Turn::Left => '<',
                        Turn::Right => '>',
                        Turn::Straight => c.dir
                    },
                    'v' => match c.next_turn {
                        Turn::Left => '>',
                        Turn::Right => '<',
                        Turn::Straight => c.dir
                    },
                    _ => panic!()
                };
                match c.next_turn {
                    Turn::Left => c.next_turn = Turn::Straight,
                    Turn::Straight => c.next_turn = Turn::Right,
                    Turn::Right => c.next_turn = Turn::Left
                }
            }
        }
        if let Some(pos) = is_crash(&cars) {
            return Some(pos);
        }
    }
    None
}

fn is_crash(cars: &Vec<Car>) -> Option<(usize, usize)> {
    for i in 0..cars.len() {
        for j in (i + 1)..cars.len() {
            if cars[i].pos == cars[j].pos {
                return Some(cars[i].pos);
            }
        }
    }
    return None;
}

fn solve(path: &Path) -> (usize, usize) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut grid = vec![];
    let mut max_w = 0;
    let mut cars = vec![];
    let mut m = HashMap::new();
    m.insert('<', '-');
    m.insert('>', '-');
    m.insert('v', '|');
    m.insert('^', '|');
    for (row, line) in lines.iter().enumerate() {
        let row_cars : Vec<Car>= line.chars().enumerate().filter(|(_, c)| ['<', '>', 'v', '^'].contains(&c)).map(|(col, c)| Car {dir: c, pos: (col, row), next_turn: Turn::Left }).collect();
        cars.extend(row_cars);
        let v : Vec<char> = line.chars().map(|c| *m.get(&c).unwrap_or(&c)).collect();
        max_w = std::cmp::max(v.len(), max_w);
        grid.push(v);
    }
    for row in &mut grid {
        row.resize(max_w, ' ');
    }
    loop {
        if grid.len() < 10 {
            draw_grid(&cars, &grid);
        }
        if let Some(pos) = tick(&mut cars, &grid) {
            return pos;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{:?}", result);
}
