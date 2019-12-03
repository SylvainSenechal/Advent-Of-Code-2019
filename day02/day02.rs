use std::io;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

fn read_input() -> Result<Vec<i32>, io::Error> {
    let mut content = String::new();
    File::open("input.txt")?.read_to_string(&mut content)?;
    let opcodes: Vec<i32> = content.split(',')
        .map(|elem| elem.trim())
        .filter(|s| !s.is_empty())
        .map(|elem| elem.parse::<i32>().unwrap())
        .collect();

    Ok(opcodes)
}

fn part1(opcodes: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut opcodes = opcodes.clone();
    opcodes[1] = noun;
    opcodes[2] = verb;
    let mut index_opcode = 0;
    loop {
        let pos1 = opcodes[index_opcode + 1] as usize;
        let pos2 = opcodes[index_opcode + 2] as usize;
        let target = opcodes[index_opcode + 3] as usize;
        match opcodes[index_opcode] {
            1 => {
                opcodes[target] = opcodes[pos1] + opcodes[pos2];
                index_opcode += 4;
            },
            2 => {
                opcodes[target] = opcodes[pos1] * opcodes[pos2];
                index_opcode += 4;
            },
            99 => break,
            _ => panic!()
        }
    }
    opcodes[0]
}

fn part2(opcodes: &Vec<i32>) -> i32 {
    let mut result = 0;
    for noun in 0..100 {
        for verb in 0..100 {
            match part1(opcodes, noun, verb) {
                19690720 => {
                    result = 100 * noun + verb;
                    break;
                },
                _ => continue
            }
        }
    };
    result
}

fn main() {
    let content = match read_input() {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e),
    };
    let now = SystemTime::now();
    let noun = 12;
    let verb = 2;
    let result1 = part1(&content, noun, verb);
    let result2 = part2(&content);
    println!("{:?}", result1);
    println!("{:?}", result2);
    println!("{:?}", now.elapsed());
}
