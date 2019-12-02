use std::io;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

fn read_input() -> Result<Vec<i32>, io::Error> {
    let mut content = String::new();
    File::open("input.txt")?.read_to_string(&mut content)?;
    let mut mass_modules: Vec<i32> = Vec::new();

    for line in content.lines() {
        mass_modules.push(line.parse::<i32>().unwrap());
    }

    Ok(mass_modules)
}

fn compute_fuel(mass: &i32) -> i32 {
    mass / 3 - 2
}

fn compute_fuel_recursive(mass: &i32) -> i32 {
    match compute_fuel(mass) {
        m if m <= 6 => m,
        m => m + compute_fuel_recursive(&m)
    }
}

fn part1(mass_modules: &Vec<i32>) -> i32 {
    let mut total_fuel: i32 = 0;
    for mass in mass_modules {
        total_fuel += compute_fuel(mass);
    }
    total_fuel
}

fn part2(mass_modules: &Vec<i32>) -> i32 {
    let mut total_fuel: i32 = 0;
    for mass in mass_modules {
        total_fuel += compute_fuel_recursive(mass);
    }
    total_fuel
}

fn main() {
    let content = match read_input() {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e),
    };
    let now = SystemTime::now();
    let result1 = part1(&content);
    let result2 = part2(&content);
    println!("{:?}", result1);
    println!("{:?}", result2);
    println!("{:?}", now.elapsed());
}
