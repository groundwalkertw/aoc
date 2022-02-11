use super::*;

pub fn task1() {
    let f = File::open("input_2.txt").unwrap();
    let reader = BufReader::new(f);
    let (mut x, mut y): (i32, i32) = (0, 0);


    for line in reader.lines() {
        let line = line.unwrap();

        let mut iterator = line.split(' ');
        
        let cmd = iterator.next().unwrap();
        let distance: i32 = iterator.next().unwrap().parse().unwrap();

        match cmd {
            "up" => {y += distance;},
            "down" => {y -= distance;},
            _ => {x += distance},
        }
        
    }
    println!("{}", x * y);
}
pub fn task2() {
    let f = File::open("input_2.txt").unwrap();
    let reader = BufReader::new(f);
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut aim: i32 = 0;


    for line in reader.lines() {
        let line = line.unwrap();

        let mut iterator = line.split(' ');
        
        let cmd = iterator.next().unwrap();
        let distance: i32 = iterator.next().unwrap().parse().unwrap();

        match cmd {
            "up" => {aim -= distance;},
            "down" => {aim += distance;},
            _ => {
                x += distance;
                y += aim * distance;
            },
        }
        
    }
    println!("{}", x * y);
}