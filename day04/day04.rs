use std::time::SystemTime;

fn part1(min: i32, max: i32) -> i32 {
    let mut nb_passwords = 0;

    // let a = (100_000..100_100).filter(check_decrease);
    // let mut a = [10, 11, 12, 13, 14, 20].iter().filter(|x| **x > 13);
    let mut a = [10, 11, 12, 13, 14, 20].iter().filter(|&val| check(val)).count();
    // let mut iter = [0, 1, 2].iter().filter(|x| **x > 1); // need two *s!

    println!("{:?}", a);
    // for password in 100_000..100_100 {
    //     let current_chars = password.to_string().chars().next().unwrap();
    //     print!("{:?}", current_chars);
    //     for c in password.to_string().chars() {
    //         // println!("{:?}", c);
    //     }
    // }
    1
}

fn check(nb: &i32) -> bool {
    println!("{:?}", nb);
    if (*nb > 10) {
        true
    } else {
        false
    }
}

// fn check_decrease(nb: &i32) -> bool {
//     true
//     // (0..5).any(|index| nb[&index] >= nb[&index - 1])
//     // for c in password.to_string().chars() {
//     //
//     // }
// }

fn sup(val: &i32) -> bool {
    if *val > 4 {true}
    else {false}
}

fn main() {
    let min = 246515;
    let max = 739105;

    // for i in 0..10 {
    //     let a = for j in 0..10 {
    //         println!("{:?}", j);
    //         if (j == 5) {
    //             break true;
    //         }
    //     };
    // }

    let now = SystemTime::now();
    let result1 = part1(min, max);

    // // let result2 = part2(&content);
    println!("{:?}", result1);
    // // println!("{:?}", result2);
    println!("{:?}", now.elapsed());
}
