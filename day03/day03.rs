use std::io;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
struct Wire {
    direction: Direction,
    length: i32
}

impl FromStr for Wire { // https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, length) = s.split_at(1);

        let length: i32 = length.parse::<i32>().unwrap();
        let direction: Direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!()
        };
        Ok(Wire {direction, length})
    }
}

fn read_input() -> Result<(Vec<Wire>, Vec<Wire>), io::Error> {
    let mut content = String::new();
    File::open("input.txt")?.read_to_string(&mut content)?;

    let wires: Vec<Vec<Wire>> = content.trim()
        .split('\n')
        .map(|wire|
            wire.split(',')
                .map(|wire_segment| Wire::from_str(wire_segment).unwrap())
                .collect()
        )
        .collect();

    Ok((wires[0].clone(), wires[1].clone()))
}

fn part1(wire1: Vec<Wire>, wire2: Vec<Wire>) -> i32 {
    let mut min_dst = 1000000;
    let mut current_pos = (0, 0);
    let mut positions = HashMap::new();
    for wire in wire1 {
        for _segment in 0..wire.length {
            match wire.direction {
                Direction::Up => {
                    current_pos.1 += 1;
                },
                Direction::Down => {
                    current_pos.1 -= 1;
                },
                Direction::Left => {
                    current_pos.0 -= 1;
                },
                Direction::Right => {
                    current_pos.0 += 1;
                }
            }
            positions.insert(current_pos, 0);
        }
    }
    let mut current_pos = (0, 0);
    for wire in wire2 {
        for _segment in 0..wire.length {
            match wire.direction {
                Direction::Up => {
                    current_pos.1 += 1;
                },
                Direction::Down => {
                    current_pos.1 -= 1;
                },
                Direction::Left => {
                    current_pos.0 -= 1;
                },
                Direction::Right => {
                    current_pos.0 += 1;
                }
            }
            // println!("cuur pos {:?}", current_pos);
            match positions.get(&current_pos) {
                Some(_) => {
                    let dst = manhattan_distance(current_pos);
                    if dst < min_dst && dst != 0 {
                        min_dst = dst;
                        println!("dst {:?}", min_dst);
                    }
                },
                None => ()
            }
        }
    }
    min_dst
}

fn manhattan_distance(position: (i32, i32)) -> i32 {
    position.0.abs() + position.1.abs()
}

fn main() {
    let wires = match read_input() {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e),
    };
    // println!("{:?}", wires);
    let now = SystemTime::now();
    let result1 = part1(wires.0, wires.1);
    // let result2 = part2(&content);
    println!("{:?}", result1);
    // println!("{:?}", result2);
    println!("{:?}", now.elapsed());
}
