use super::*;

const N: usize = 5;

#[derive(Debug)]
struct Checker {
    marked: bool,
    num: u32,
}
#[derive(Debug)]
struct Board (Vec<Vec<Checker>>);

impl Board {
    fn mark(&mut self, num: u32) {
        for i in 0..N {
            for j in 0..N {
                if (self.0)[i][j].num == num {
                    (self.0)[i][j].marked = true;
                }
            }
        }
    }

    fn col_check(&self, i: usize) -> bool {
        for j in 0..N {
            if !self.0[j][i].marked {
                return false;
            }
        }
        true
    }
    fn row_check(&self, i: usize) -> bool {
        for j in 0..N {
            if !self.0[i][j].marked {
                return false;
            }
        }
        true
    }

    fn check(&self) -> bool {
        for i in 0..N {
            if self.col_check(i) || self.row_check(i) {
                return true;
            }
        }
        false
    }
    fn do_sum(&self) -> u32 {
        let iter = self.0.iter().flatten();
        let mut sum = 0;
        for checker in iter {
            if !checker.marked {
                sum += checker.num; 
            }
        }
        sum
    }
}
pub fn task1() {
    let f = File::open("input_4.txt").unwrap();
    let reader = BufReader::new(f);

    let mut line_iter = reader.lines();

    let line_1st = line_iter.next().unwrap().unwrap();

    let numbers: Vec<_> = line_1st.split(',').map(|num| num.parse::<u32>().unwrap()).collect();

    let mut counter: u32 = 0;

    let mut boards: Vec<Board> = Vec::new();
    
    for line in line_iter {
        counter += 1;
        if counter % 6 == 1 {            
            boards.push(Board(Vec::new()));
            continue;
        } else {
            let line = line.unwrap();
            let row: Vec<_> = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|num| Checker{marked: false, num: num.parse::<u32>().unwrap()})
                .collect();
            boards.last_mut().unwrap().0.push(row);
        }
    }

    let mut sum= 0;
    let mut num = 0;

    'outer: for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.check() {
                sum = board.do_sum();
                num = number;
                break 'outer;
            }
        }
    }

    println!("{}", sum * num);
}
pub fn task2() {
        let f = File::open("input_4.txt").unwrap();
    let reader = BufReader::new(f);

    let mut line_iter = reader.lines();

    let line_1st = line_iter.next().unwrap().unwrap();

    let numbers: Vec<_> = line_1st.split(',').map(|num| num.parse::<u32>().unwrap()).collect();

    let mut counter: u32 = 0;

    let mut boards: Vec<Board> = Vec::new();
    
    for line in line_iter {
        counter += 1;
        if counter % 6 == 1 {            
            boards.push(Board(Vec::new()));
            continue;
        } else {
            let line = line.unwrap();
            let row: Vec<_> = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|num| Checker{marked: false, num: num.parse::<u32>().unwrap()})
                .collect();
            boards.last_mut().unwrap().0.push(row);
        }
    }

    let mut sum= 0;
    let mut score = 0;

    

    for number in numbers {
        let mut record: Vec<usize> = Vec::new();       
        for i in 0..boards.len() {
            boards[i].mark(number);
            if boards[i].check() {
                sum = boards[i].do_sum();
                score = number * sum;
                record.push(i);
            }
        }
        for j in 0..record.len() {
            boards.remove(record[record.len() - 1 - j]);
        }
    }

    println!("{}", score);
}


